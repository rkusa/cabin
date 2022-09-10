use std::marker::PhantomData;

use super::{HashTree, IntoView};
use crate::{Render, View};

pub fn any<V, S>(view: V) -> AnyView<S>
where
    V: View<S> + 'static,
    V::Renderer: 'static,
{
    AnyView {
        view: Box::new(|hash_tree: &mut HashTree| {
            view.into_renderer(hash_tree)
                .map::<Box<dyn Render>, _>(|r| Box::new(r))
        }),
        marker: PhantomData,
    }
}

type ViewBoxRenderer = dyn FnOnce(&mut HashTree) -> Option<Box<dyn Render>>;

pub struct AnyView<S> {
    view: Box<ViewBoxRenderer>,
    marker: PhantomData<S>,
}

impl<S> View<S> for AnyView<S> {
    type Renderer = Box<dyn Render>;

    fn into_renderer(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        (self.view)(hash_tree)
    }
}

impl<S> IntoView<AnyView<S>, S> for AnyView<S> {
    fn into_view(self) -> AnyView<S> {
        self
    }
}

impl<R> Render for Box<R>
where
    R: Render + ?Sized,
{
    fn render(&self, out: &mut dyn std::fmt::Write, is_update: bool) -> std::fmt::Result {
        R::render(self, out, is_update)
    }
}
