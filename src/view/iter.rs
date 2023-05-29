use std::marker::PhantomData;

pub use super::View;
use crate::render::Renderer;

pub trait IteratorExt<Iter, V, Ev> {
    fn into_view(self) -> IteratorView<Iter, V, Ev>;
}

impl<Iter, V, Ev> IteratorExt<Iter::IntoIter, V, Ev> for Iter
where
    Iter: IntoIterator<Item = V>,
    V: View<Ev>,
{
    fn into_view(self) -> IteratorView<Iter::IntoIter, V, Ev> {
        IteratorView {
            iter: self.into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct IteratorView<Iter, V, Ev> {
    iter: Iter,
    marker: PhantomData<(V, Ev)>,
}

impl<Iter, V, Ev> View<Ev> for IteratorView<Iter, V, Ev>
where
    Iter: Iterator<Item = V>,
    V: View<Ev>,
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
