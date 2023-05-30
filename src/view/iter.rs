use std::marker::PhantomData;

pub use super::View;
use crate::render::Renderer;

pub trait IteratorExt<Iter, V> {
    fn into_view(self) -> IteratorView<Iter, V>;
}

impl<Iter, V> IteratorExt<Iter::IntoIter, V> for Iter
where
    Iter: IntoIterator<Item = V>,
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
    marker: PhantomData<(V)>,
}

impl<Iter, V> View for IteratorView<Iter, V>
where
    Iter: Iterator<Item = V>,
    V: View,
{
    async fn render(self, mut r: Renderer) -> Result<Renderer, crate::Error> {
        for i in self.iter {
            let fut = i.render(r);
            r = fut.await?;
        }
        Ok(r)
    }

    // TODO: any way to prime without consuming the iterator?
}
