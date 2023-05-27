use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

pub use super::View;
use crate::render::Renderer;

pub trait IteratorExt<Iter, V> {
    fn into_view(self) -> IteratorView<Iter, V>;
}

impl<Iter, V> IteratorExt<Iter::IntoIter, V> for Iter
where
    Iter: IntoIterator<Item = V>,
    // TODO: remove `+ 'static` once removing away from boxed future
    Iter::IntoIter: Send + 'static,
    V: View,
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
    Iter: Iterator<Item = V> + 'static,
    V: View,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

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
