use std::fmt;
use std::marker::PhantomData;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<I, V, M> IntoView<IteratorView<I::IntoIter, V, M>, M> for I
where
    I: IntoIterator<Item = V>,
    I::IntoIter: Clone,
    V: View<M>,
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
    I: Iterator<Item = V> + Clone,
    V: View<M>,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        for i in self.iter.clone() {
            i.render(r)?;
        }
        Ok(())
    }
}
