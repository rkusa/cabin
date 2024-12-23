use std::future::Future;

use super::RenderFuture;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Html, Raw};
use crate::render::Renderer;
use crate::View;

pub struct UpdateView<V> {
    view: V,
    behaviour: Behaviour,
}

enum Behaviour {
    Hidden,
    ContentOnly,
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
}

impl<El, A, V> View for UpdateView<Html<El, A, V>>
where
    El: Send + 'static,
    A: Attributes,
    V: View,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        if r.is_update() {
            match self.behaviour {
                Behaviour::Hidden => RenderFuture::Ready(Some(Ok(r))),
                Behaviour::ContentOnly => self.view.content.render(r, include_hash),
            }
        } else {
            self.view.render(r, include_hash)
        }
    }

    fn prime(&mut self) -> impl Future<Output = ()> + Send {
        self.view.prime()
    }
}

impl View for UpdateView<Raw> {
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        if r.is_update() {
            RenderFuture::Ready(Some(Ok(r)))
        } else {
            self.view.render(r, include_hash)
        }
    }

    fn prime(&mut self) -> impl Future<Output = ()> + Send {
        self.view.prime()
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
