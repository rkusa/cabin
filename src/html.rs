mod attributes;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;

use serde::Deserialize;

use crate::action::EventAction;
use crate::view::{HashTree, View};
use crate::Action;

use self::attributes::{Attribute, Attributes};

pub fn div<S>() -> HtmlTagBuilder<S, ()> {
    HtmlTagBuilder {
        tag: "div",
        ..Default::default()
    }
}

pub fn button<S>() -> HtmlTagBuilder<S, ()> {
    HtmlTagBuilder {
        tag: "button",
        ..Default::default()
    }
}

pub fn custom<S>(tag: &'static str) -> HtmlTagBuilder<S, ()> {
    HtmlTagBuilder {
        tag,
        ..Default::default()
    }
}

pub struct HtmlTag<V, S, A> {
    builder: HtmlTagBuilder<S, A>,
    content: V,
}

pub struct HtmlTagBuilder<S, A> {
    tag: &'static str,
    attrs: A,
    on_click: Option<Action<S>>,
    on_input: Option<EventAction<S, InputEvent>>,
}

#[derive(Deserialize)]
#[non_exhaustive]
pub struct InputEvent {
    pub value: String,
}

impl<S, A> HtmlTagBuilder<S, A>
where
    A: Attributes,
{
    pub fn attr(
        self,
        name: &'static str,
        value: impl Into<Cow<'static, str>>,
    ) -> HtmlTagBuilder<S, impl Attributes> {
        HtmlTagBuilder {
            tag: self.tag,
            attrs: Attribute::new(name, value, self.attrs),
            on_click: self.on_click,
            on_input: self.on_input,
        }
    }

    // TODO: not available for all tags (e.g. only for buttons)
    pub fn on_click(mut self, action: Action<S>) -> Self {
        self.on_click = Some(action);
        self
    }

    // TODO: not available for all tags (e.g. only for inputs)
    pub fn on_input(mut self, action: EventAction<S, InputEvent>) -> Self {
        self.on_input = Some(action);
        self
    }

    pub fn content<V: View<S>>(self, content: V) -> HtmlTag<V, S, A> {
        HtmlTag {
            builder: self,
            content,
        }
    }
}

impl<V, S, A> View<S> for HtmlTag<V, S, A>
where
    V: View<S>,
    A: Attributes,
{
    fn render(mut self, hash_tree: &mut HashTree, mut out: impl Write) -> fmt::Result {
        let mut node = hash_tree.node();
        node.write(self.builder.tag.as_bytes());

        write!(&mut out, "<{}", self.builder.tag)?;

        if let Some(on_click) = self.builder.on_click.take() {
            // TODO: avoid allocation
            let action = format!("{}::{}", on_click.module, on_click.name);
            Attribute::new("data-click", action, ()).render(&mut node, &mut out)?;
        }

        if let Some(on_input) = self.builder.on_input.take() {
            // TODO: avoid allocation
            let action = format!("{}::{}", on_input.module, on_input.name);
            Attribute::new("data-input", action, ()).render(&mut node, &mut out)?;
        }

        self.builder.attrs.render(&mut node, &mut out)?;

        // Handle void elements. Content is simply ignored.
        // https://html.spec.whatwg.org/multipage/syntax.html#elements-2
        if is_void_element(self.builder.tag) {
            write!(&mut out, "/>")?;
            return Ok(());
        }

        // TODO: get away without the extra string here (problem is the data-hash)?
        let mut inner = String::new();
        self.content.render(node.content(), &mut inner)?;

        let hash = node.end();
        write!(&mut out, r#" data-hash="{}""#, hash)?;

        write!(&mut out, ">{}</{}>", inner, self.builder.tag)?;
        Ok(())
    }
}

fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "source"
            | "track"
            | "wbr"
    )
}

impl<S, A> View<S> for HtmlTagBuilder<S, A>
where
    A: Attributes,
{
    fn render(self, hash_tree: &mut HashTree, out: impl Write) -> fmt::Result {
        HtmlTag {
            builder: self,
            content: (),
        }
        .render(hash_tree, out)
    }
}

impl<S> Default for HtmlTagBuilder<S, ()> {
    fn default() -> Self {
        Self {
            tag: "div",
            attrs: (),
            on_click: None,
            on_input: None,
        }
    }
}
