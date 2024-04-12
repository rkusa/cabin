use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

use tokio::task::JoinHandle;

use super::RenderFuture;
pub use super::View;
use crate::error::InternalError;
use crate::render::Renderer;
use crate::scope::Scope;

pub trait FutureExt<F, V>
where
    F: IntoFuture<Output = V>,
    V: View,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V>;
}

impl<F, V> FutureExt<F, V> for F
where
    F: IntoFuture<Output = V>,
    V: View,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V> {
        FutureView {
            key: Scope::key(),
            state: State::Stored(Box::pin(self.into_future())),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, V>
where
    F: Future,
{
    key: Option<u32>,
    state: State<F>,
    marker: PhantomData<V>,
}

enum State<F>
where
    F: Future,
{
    // Explicitly put future on heap (Box) to prevent stack overflow for very large futures.
    Stored(Pin<Box<F>>),
    Primed(JoinHandle<F::Output>),
    Intermediate,
}

impl<F, V> View for FutureView<F, V>
where
    F: Future<Output = V> + 'static,
    V: View + 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        RenderFuture::Future(Box::pin(async move {
            let view = if let Some(key) = self.key {
                Scope::keyed(key, async {
                    match self.state {
                        State::Stored(f) => Ok(f.await),
                        State::Primed(f) => f
                            .await
                            .map_err(InternalError::Join)
                            .map_err(crate::Error::from),
                        State::Intermediate => unreachable!(),
                    }
                })
                .await?
            } else {
                match self.state {
                    State::Stored(f) => f.await,
                    State::Primed(f) => f.await.map_err(InternalError::Join)?,
                    State::Intermediate => unreachable!(),
                }
            };
            view.render(r, include_hash).await
        }))
    }

    fn prime(&mut self) {
        let key = self.key;
        let s = std::mem::replace(&mut self.state, State::Intermediate);
        let _ = std::mem::replace(
            &mut self.state,
            match s {
                State::Stored(f) => State::Primed(Scope::spawn_local(async move {
                    let mut view = if let Some(key) = key {
                        Scope::keyed(key, f).await
                    } else {
                        f.await
                    };
                    view.prime();
                    view
                })),
                State::Primed(f) => State::Primed(f),
                State::Intermediate => unreachable!(),
            },
        );
    }
}
