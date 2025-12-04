use std::future::{Future, IntoFuture};
use std::marker::PhantomData;

use super::RenderFuture;
pub use super::View;
use crate::render::Renderer;

pub trait FutureExt<'v, F, V>
where
    F: IntoFuture<Output = V>,
    V: View<'v>,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V>;
}

impl<'v, F, V> FutureExt<'v, F, V> for F
where
    F: IntoFuture<Output = V>,
    V: View<'v>,
{
    fn into_view(self) -> FutureView<F::IntoFuture, V> {
        FutureView {
            future: self.into_future(),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, V>
where
    F: Future,
{
    future: F,
    marker: PhantomData<V>,
}

impl<'v, F, V> View<'v> for FutureView<F, V>
where
    F: Future<Output = V> + 'v,
    V: View<'v>,
{
    fn render(self, r: Renderer) -> RenderFuture<'v> {
        RenderFuture::Future(Box::pin(async move {
            let view = self.future.await;
            view.render(r).await
        }))
    }
}
