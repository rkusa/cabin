use std::fmt;
use std::marker::PhantomData;

use super::IntoView;
pub use super::View;
use crate::render::Renderer;

impl<I, V, S> IntoView<IteratorView<I::IntoIter, V, S>, S> for I
where
    I: IntoIterator<Item = V>,
    I::IntoIter: Clone,
    V: View<S>,
{
    fn into_view(self) -> IteratorView<I::IntoIter, V, S> {
        IteratorView {
            iter: self.into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct IteratorView<I, V, S> {
    iter: I,
    marker: PhantomData<(V, S)>,
}

impl<I, V, S> View<S> for IteratorView<I, V, S>
where
    I: Iterator<Item = V> + Clone,
    V: View<S>,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        for i in self.iter.clone() {
            i.render(r)?;
        }
        Ok(())
    }
}
