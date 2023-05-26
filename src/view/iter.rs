use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<Iter, V> IntoView<IteratorView<Iter::IntoIter, V>> for Iter
where
    Iter: IntoIterator<Item = V>,
    // TODO: remove `+ 'static` once removing away from boxed future
    Iter::IntoIter: Send + 'static,
    V: View + Send,
{
    fn into_view(self) -> IteratorView<Iter::IntoIter, V> {
        IteratorView {
            iter: self.into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct IteratorView<Iter, V> {
    iter: Iter,
    marker: PhantomData<V>,
}

impl<Iter, V> View for IteratorView<Iter, V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    Iter: Iterator<Item = V> + Send + 'static,
    V: View + Send,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>;

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
