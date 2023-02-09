use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<F, I, V> IntoView<FutureView<F::IntoFuture, I, V>> for F
where
    F: IntoFuture<Output = I>,
    // TODO: remove `+ 'static` once removing away from boxed future
    F::IntoFuture: Send + 'static,
    I: IntoView<V> + Send,
    V: View + Send,
{
    fn into_view(self) -> FutureView<F::IntoFuture, I, V> {
        FutureView {
            future: self.into_future(),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, I, V> {
    future: F,
    marker: PhantomData<(I, V)>,
}

impl<F, I, V> View for FutureView<F, I, V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    F: Future<Output = I> + Send + 'static,
    I: IntoView<V> + Send,
    V: View + Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async move {
            let view = self.future.await.into_view();
            view.render(r).await
        })
    }
}
