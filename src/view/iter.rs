use std::fmt::Write;
use std::marker::PhantomData;

use super::IntoView;
pub use super::View;
use crate::Render;

impl<I, V, S> IntoView<IteratorView<I, V, S>, S> for I
where
    I: Iterator<Item = V>,
    V: View<S>,
{
    fn into_view(self) -> IteratorView<I, V, S> {
        IteratorView::<I, V, S> {
            iter: self,
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
    I: Iterator<Item = V>,
    V: View<S>,
{
    type Render = IteratorRenderer<V, S>;

    fn prepare(self, hash_tree: &mut super::HashTree) -> Option<Self::Render> {
        Some(IteratorRenderer {
            renderers: self.iter.map(|v| v.prepare(hash_tree)).collect(),
        })
    }
}

pub struct IteratorRenderer<V: View<S>, S> {
    renderers: Vec<Option<V::Render>>,
}

impl<V: View<S>, S> Render for IteratorRenderer<V, S> {
    fn render(&self, mut out: &mut dyn Write, is_update: bool) -> std::fmt::Result {
        for r in &self.renderers {
            r.render(&mut out, is_update)?;
        }
        Ok(())
    }
}
