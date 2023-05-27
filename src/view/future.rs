use std::future::{Future, IntoFuture};
use std::marker::PhantomData;

pub use super::View;
use crate::render::Renderer;

pub trait FutureExt<F, V, Ev> {
    fn into_view(self) -> FutureView<F, V, Ev>;
}

impl<F, V, Ev> FutureExt<F::IntoFuture, V, Ev> for F
where
    F: IntoFuture<Output = V>,
    V: View<Ev>,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V, Ev> {
        FutureView {
            future: self.into_future(),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, V, Ev> {
    future: F,
    marker: PhantomData<(V, Ev)>,
}

impl<F, V, Ev> View<Ev> for FutureView<F, V, Ev>
where
    F: Future<Output = V>,
    V: View<Ev> + Send,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let view = self.future.await;
        view.render(r).await
    }
}
