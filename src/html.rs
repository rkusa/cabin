mod attributes;
pub mod events;

use std::borrow::Cow;
use std::fmt;

use serde::Serialize;

use self::attributes::{Attribute, Attributes};
use self::events::InputEvent;
use crate::render::{is_void_element, Renderer};
use crate::view::{IntoView, View};

pub fn div<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("div", content)
}

pub fn ul<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("ul", content)
}

pub fn li<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("li", content)
}

pub fn button<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("button", content)
}

pub fn input<M>() -> Html<(), M, ()> {
    custom("input", ())
}

pub fn custom<V: View<M>, M>(tag: &'static str, content: impl IntoView<V, M>) -> Html<V, M, ()> {
    Html {
        tag: HtmlTag {
            tag,
            attrs: (),
            on_click: None,
            on_input: None,
        },
        content: content.into_view(),
    }
}

struct HtmlTag<M, A> {
    tag: &'static str,
    attrs: A,
    on_click: Option<M>,
    on_input: Option<M>,
}

pub struct Html<V, M, A> {
    tag: HtmlTag<M, A>,
    content: V,
}

impl<V, M, A> Html<V, M, A> {
    pub fn attr<'a>(
        self,
        name: &'static str,
        value: impl Into<Cow<'a, str>>,
    ) -> Html<V, M, impl Attributes + 'a>
    where
        A: Attributes + 'a,
    {
        Html {
            tag: HtmlTag {
                tag: self.tag.tag,
                attrs: Attribute::new(name, value, self.tag.attrs),
                on_click: self.tag.on_click,
                on_input: self.tag.on_input,
            },
            content: self.content,
        }
    }

    // TODO: not available for all tags (e.g. only for buttons)
    pub fn on_click(mut self, message: M) -> Self {
        self.tag.on_click = Some(message);
        self
    }

    // TODO: not available for all tags (e.g. only for inputs)
    pub fn on_input(mut self, message: impl FnOnce(InputEvent) -> M) -> Self {
        self.tag.on_input = Some(message(InputEvent::default()));
        self
    }
}

impl<V, M, A> View<M> for Html<V, M, A>
where
    V: View<M>,
    M: Serialize,
    A: Attributes,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        let mut el = r.element(self.tag.tag)?;

        if let Some(on_click) = &self.tag.on_click {
            // TODO: avoid allocation
            // TODO: unwrap
            let action = serde_json::to_string(&on_click).unwrap();
            el.attribute("data-click", &action)?;
        }

        if let Some(on_input) = &self.tag.on_input {
            // TODO: avoid allocation
            // TODO: unwrap
            let action = serde_json::to_string(&on_input).unwrap();
            el.attribute("data-input", &action)?;
        }

        self.tag.attrs.render(&mut el)?;

        if !is_void_element(self.tag.tag) {
            let content = el.content()?;
            self.content.render(content)?;
        }

        el.end()?;

        Ok(())
    }
}

impl<V, M, A> IntoView<Html<V, M, A>, M> for Html<V, M, A>
where
    V: View<M>,
    M: Serialize,
    A: Attributes,
{
    fn into_view(self) -> Html<V, M, A> {
        self
    }
}
