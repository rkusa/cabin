use std::fmt;
use std::future::{Future, IntoFuture};
use std::marker::PhantomData;
use std::pin::Pin;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<F, I, V, M> IntoView<FutureView<F::IntoFuture, I, V, M>, M> for F
where
    F: IntoFuture<Output = I>,
    // TODO: remove `+ 'static` once removing away from boxed future
    F::IntoFuture: Send + 'static,
    I: IntoView<V, M> + Send,
    V: View<M> + Send,
    M: Send,
{
    fn into_view(self) -> FutureView<F::IntoFuture, I, V, M> {
        FutureView {
            future: self.into_future(),
            marker: PhantomData,
        }
    }
}

pub struct FutureView<F, I, V, M> {
    future: F,
    marker: PhantomData<(I, V, M)>,
}

impl<F, I, V, M> View<M> for FutureView<F, I, V, M>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    F: Future<Output = I> + Send + 'static,
    I: IntoView<V, M> + Send,
    V: View<M> + Send,
    M: Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async move {
            let view = self.future.await.into_view();
            view.render(r).await
        })
    }
}
