use std::future::{Future, IntoFuture};
use std::marker::PhantomData;

use tokio::task::JoinHandle;

pub use super::View;
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
            state: State::Stored(self.into_future()),
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
    Stored(F),
    Primed(JoinHandle<F::Output>),
    Intermediate,
}

impl<F, V> View for FutureView<F, V>
where
    F: Future<Output = V> + 'static,
    V: View + 'static,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let view = if let Some(key) = self.key {
            Scope::keyed(key, async {
                match self.state {
                    State::Stored(f) => f.await,
                    State::Primed(f) => f.await.unwrap(), // TODO: handle JoinError?
                    State::Intermediate => unreachable!(),
                }
            })
            .await
        } else {
            match self.state {
                State::Stored(f) => f.await,
                State::Primed(f) => f.await.unwrap(), // TODO: handle JoinError?
                State::Intermediate => unreachable!(),
            }
        };
        view.render(r).await
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
