use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

pub use super::View;
use crate::render::Renderer;

pub trait FutureExt<F, V> {
    fn into_view(self) -> FutureView<F, V>;
}

impl<F, V> FutureExt<F::IntoFuture, V> for F
where
    F: IntoFuture<Output = V>,
    // TODO: remove `+ 'static` once removing away from boxed future
    F::IntoFuture: 'static,
    V: View + Send,
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
    // TODO: remove `+ 'static` once removing away from boxed future
    F: Future<Output = V> + 'static,
    V: View + Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async move {
            let view = self.future.await;
            view.render(r).await
        })
    }
}
