use super::RenderFuture;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Common, Html, Raw};
use crate::render::Renderer;
use crate::{View, h};

pub struct UpdateView<V> {
    view: V,
    behaviour: Behaviour,
}

enum Behaviour {
    Hidden,
    ContentOnly,
    Template { id: &'static str },
}

impl<V> UpdateView<V> {
    pub fn hidden_on_update(view: V) -> Self {
        Self {
            view,
            behaviour: Behaviour::Hidden,
        }
    }

    pub fn content_only_on_update(view: V) -> Self {
        Self {
            view,
            behaviour: Behaviour::ContentOnly,
        }
    }

    pub fn template_on_update(id: &'static str, view: V) -> Self {
        Self {
            view,
            behaviour: Behaviour::Template { id },
        }
    }
}

impl<El, A> View for UpdateView<Html<El, A>>
where
    El: Send + 'static,
    A: Attributes,
{
    fn render(self, r: Renderer) -> RenderFuture {
        if r.is_update() {
            match self.behaviour {
                Behaviour::Hidden => RenderFuture::Ready(Ok(r)),
                Behaviour::ContentOnly => self.view.content.render(r),
                Behaviour::Template { id } => h::template(self.view.content).id(id).render(r),
            }
        } else {
            self.view.render(r)
        }
    }
}

impl View for UpdateView<Raw> {
    fn render(self, r: Renderer) -> RenderFuture {
        if r.is_update() {
            RenderFuture::Ready(Ok(r))
        } else {
            self.view.render(r)
        }
    }
}

impl<V: WithAttribute> WithAttribute for UpdateView<V> {
    type Output<T>
        = UpdateView<V::Output<T>>
    where
        T: Attributes;

    fn with_attribute<T: Attributes>(self, attr: T) -> Self::Output<T> {
        UpdateView {
            view: self.view.with_attribute(attr),
            behaviour: self.behaviour,
        }
    }

    fn get_attribute<T: 'static>(&self) -> Option<&T> {
        self.view.get_attribute()
    }

    fn get_attribute_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.view.get_attribute_mut()
    }
}
