use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<I, V, M> IntoView<IteratorView<I::IntoIter, V, M>, M> for I
where
    I: IntoIterator<Item = V>,
    // TODO: remove `+ 'static` once removing away from boxed future
    I::IntoIter: Send + 'static,
    V: View<M> + Send,
    M: Send,
{
    fn into_view(self) -> IteratorView<I::IntoIter, V, M> {
        IteratorView {
            iter: self.into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct IteratorView<I, V, M> {
    iter: I,
    marker: PhantomData<(V, M)>,
}

impl<I, V, M> View<M> for IteratorView<I, V, M>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    I: Iterator<Item = V> + Send + 'static,
    V: View<M> + Send,
    M: Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, mut r: Renderer) -> Self::Future {
        Box::pin(async move {
            for i in self.iter {
                let fut = i.render(r);
                r = fut.await?;
            }
            Ok(r)
        })
    }
}
