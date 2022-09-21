mod attributes;

use std::borrow::Cow;
use std::fmt;

use serde::Deserialize;

use self::attributes::{Attribute, Attributes};
use crate::action::EventAction;
use crate::render::{is_void_element, Renderer};
pub use crate::view::text::text;
use crate::view::{IntoView, View};
use crate::Action;

pub fn div<V: View<S>, S>(content: impl IntoView<V, S>) -> Html<V, S, ()> {
    custom("div", content)
}

pub fn ul<V: View<S>, S>(content: impl IntoView<V, S>) -> Html<V, S, ()> {
    custom("ul", content)
}

pub fn li<V: View<S>, S>(content: impl IntoView<V, S>) -> Html<V, S, ()> {
    custom("li", content)
}

pub fn button<V: View<S>, S>(content: impl IntoView<V, S>) -> Html<V, S, ()> {
    custom("button", content)
}

pub fn input<S>() -> Html<(), S, ()> {
    custom("input", ())
}

pub fn custom<V: View<S>, S>(tag: &'static str, content: impl IntoView<V, S>) -> Html<V, S, ()> {
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

struct HtmlTag<S, A> {
    tag: &'static str,
    attrs: A,
    on_click: Option<Action<S>>,
    on_input: Option<EventAction<S, InputEvent>>,
}

pub struct Html<V, S, A> {
    tag: HtmlTag<S, A>,
    content: V,
}

#[derive(Deserialize)]
#[non_exhaustive]
pub struct InputEvent {
    pub value: String,
}

impl<V, S, A> Html<V, S, A> {
    pub fn attr(
        self,
        name: &'static str,
        value: impl Into<Cow<'static, str>>,
    ) -> Html<V, S, impl Attributes>
    where
        A: Attributes,
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
    pub fn on_click(mut self, action: Action<S>) -> Self {
        self.tag.on_click = Some(action);
        self
    }

    // TODO: not available for all tags (e.g. only for inputs)
    pub fn on_input(mut self, action: EventAction<S, InputEvent>) -> Self {
        self.tag.on_input = Some(action);
        self
    }
}

impl<V, S, A> View<S> for Html<V, S, A>
where
    V: View<S>,
    A: Attributes,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        let mut el = r.element(self.tag.tag)?;

        if let Some(on_click) = &self.tag.on_click {
            // TODO: avoid allocation
            let action = format!("{}::{}", on_click.module, on_click.name);
            el.attribute("data-click", &action)?;
        }

        if let Some(on_input) = &self.tag.on_input {
            // TODO: avoid allocation
            let action = format!("{}::{}", on_input.module, on_input.name);
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

impl<V, S, A> IntoView<Html<V, S, A>, S> for Html<V, S, A>
where
    V: View<S>,
    A: Attributes,
{
    fn into_view(self) -> Html<V, S, A> {
        self
    }
}
