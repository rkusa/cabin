use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<Iter, I, V> IntoView<IteratorView<Iter::IntoIter, I, V>> for Iter
where
    Iter: IntoIterator<Item = I>,
    // TODO: remove `+ 'static` once removing away from boxed future
    Iter::IntoIter: Send + 'static,
    I: IntoView<V> + Send,
    V: View + Send,
{
    fn into_view(self) -> IteratorView<Iter::IntoIter, I, V> {
        IteratorView {
            iter: self.into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct IteratorView<Iter, I, V> {
    iter: Iter,
    marker: PhantomData<(I, V)>,
}

impl<Iter, I, V> View for IteratorView<Iter, I, V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    Iter: Iterator<Item = I> + Send + 'static,
    I: IntoView<V> + Send,
    V: View + Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>;

    fn render(self, mut r: Renderer) -> Self::Future {
        Box::pin(async move {
            for i in self.iter {
                let fut = i.into_view().render(r);
                r = fut.await?;
            }
            Ok(r)
        })
    }
}
