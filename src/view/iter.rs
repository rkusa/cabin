use std::marker::PhantomData;

use crate::Render;

pub use super::View;

// TODO: using iter directly in component would be cooler
pub fn list<I, F, T, V, S>(iter: I, map: F) -> impl View<S>
where
    I: Iterator<Item = T>,
    F: FnMut(T) -> V,
    V: View<S>,
{
    IteratorView::<I, F, T, V, S> {
        iter,
        map,
        marker: PhantomData,
    }
}

pub struct IteratorView<I, F, T, V, S> {
    iter: I,
    map: F,
    marker: PhantomData<(T, V, S)>,
}

impl<I, F, T, V, S> View<S> for IteratorView<I, F, T, V, S>
where
    I: Iterator<Item = T>,
    F: FnMut(T) -> V,
    V: View<S>,
{
    type Renderer = IteratorRenderer<V, S>;

    fn render(self, hash_tree: &mut super::HashTree) -> Option<Self::Renderer> {
        Some(IteratorRenderer {
            renderers: self
                .iter
                .map(self.map)
                .map(|v| v.render(hash_tree))
                .collect(),
        })
    }
}

pub struct IteratorRenderer<V: View<S>, S> {
    renderers: Vec<Option<V::Renderer>>,
}

impl<V: View<S>, S> Render for IteratorRenderer<V, S> {
    fn render(self, mut out: impl std::fmt::Write) -> std::fmt::Result {
        for r in self.renderers {
            r.render(&mut out)?;
        }
        Ok(())
    }
}
