use std::future::{Future, IntoFuture};
use std::marker::PhantomData;

pub use super::View;
use crate::render::Renderer;

pub trait FutureExt<F, V> {
    fn into_view(self) -> FutureView<F, V>;
}

impl<F, V> FutureExt<F::IntoFuture, V> for F
where
    F: IntoFuture<Output = V>,
    V: View,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V> {
        FutureView {
            future: self.into_future(),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, V> {
    future: F,
    marker: PhantomData<V>,
}

impl<F, V> View for FutureView<F, V>
where
    F: Future<Output = V>,
    V: View + Send,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let view = self.future.await;
        view.render(r).await
    }
}
