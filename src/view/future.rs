use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

use super::RenderFuture;
pub use super::View;
use crate::render::Renderer;

pub trait FutureExt<F, V>
where
    F: IntoFuture<Output = V> + Send,
    F::IntoFuture: Send,
    V: View,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V>;
}

impl<F, V> FutureExt<F, V> for F
where
    F: IntoFuture<Output = V> + Send,
    F::IntoFuture: Send,
    V: View,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V> {
        FutureView {
            state: State::Stored(Box::pin(self.into_future())),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, V>
where
    F: Future,
{
    state: State<F>,
    marker: PhantomData<V>,
}

enum State<F>
where
    F: Future,
{
    // Explicitly put future on heap (Box) to prevent stack overflow for very large futures.
    Stored(Pin<Box<F>>),
    Primed(Result<F::Output, crate::Error>),
    Intermediate,
}

impl<F, V> View for FutureView<F, V>
where
    F: Future<Output = V> + Send + 'static,
    V: View,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        RenderFuture::Future(Box::pin(async move {
            let view = match self.state {
                State::Stored(f) => Ok(f.await),
                State::Primed(result) => result,
                State::Intermediate => unreachable!(),
            }?;
            view.render(r, include_hash).await
        }))
    }

    async fn prime(&mut self) {
        let s = std::mem::replace(&mut self.state, State::Intermediate);
        let _ = std::mem::replace(
            &mut self.state,
            match s {
                State::Stored(f) => State::Primed({
                    let mut view = f.await;
                    view.prime().await;
                    Ok(view)
                }),
                State::Primed(view) => State::Primed(view),
                State::Intermediate => unreachable!(),
            },
        );
    }
}
