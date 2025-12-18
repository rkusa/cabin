use super::RenderFuture;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Html, Raw};
use crate::render::Renderer;
use crate::style::collector::StyleDelegate;
use crate::style::{Style, StyleDefinition, StyleModifier, SubStyle};

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
    fn render(mut self, r: Renderer) -> RenderFuture {
        if r.is_update() {
            match self.behaviour {
                Behaviour::Hidden => RenderFuture::Ready(Ok(r)),
                Behaviour::ContentOnly => self.view.content.render(r),
                Behaviour::Template { id } => {
                    self.view.change_tag("template");
                    self.view
                        .with_attribute(crate::html::elements::common::Id(id.into()))
                        .render(r)
                }
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

impl<El, A> Style for UpdateView<Html<El, A>> {
    fn style_mut(&mut self) -> &mut StyleDefinition {
        self.view.style_mut()
    }
}

impl<El, A> SubStyle for UpdateView<Html<El, A>> {
    fn style_mut_for(&mut self, modifier: StyleModifier) -> &mut StyleDefinition {
        self.view.style_mut_for(modifier)
    }

    fn substyle<F: for<'a> FnOnce(StyleDelegate<'a>) -> StyleDelegate<'a>>(
        mut self,
        modifier: StyleModifier,
        f: F,
    ) -> Self {
        self.view = self.view.substyle(modifier, f);
        self
    }
}
