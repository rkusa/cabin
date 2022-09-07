mod attributes;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;

use serde::Deserialize;

use self::attributes::{Attribute, Attributes};
use crate::action::EventAction;
pub use crate::view::text::text;
use crate::view::{HashTree, View};
use crate::{Action, Render};

pub fn div<V: View<S>, S>(content: V) -> Html<V, S, ()> {
    custom("div", content)
}

pub fn ul<V: View<S>, S>(content: V) -> Html<V, S, ()> {
    custom("ul", content)
}

pub fn li<V: View<S>, S>(content: V) -> Html<V, S, ()> {
    custom("li", content)
}

pub fn button<V: View<S>, S>(content: V) -> Html<V, S, ()> {
    custom("button", content)
}

pub fn input<S>() -> Html<(), S, ()> {
    custom("button", ())
}

pub fn custom<V: View<S>, S>(tag: &'static str, content: V) -> Html<V, S, ()> {
    Html {
        tag: HtmlTag {
            tag,
            attrs: (),
            on_click: None,
            on_input: None,
        },
        content,
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
    type Renderer = HtmlTagRenderer<S, A, V::Renderer>;

    fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        let mut node = hash_tree.node();
        node.write(self.tag.tag.as_bytes());

        if let Some(on_click) = &self.tag.on_click {
            // TODO: avoid double allocation (again in render)
            let action = format!("{}::{}", on_click.module, on_click.name);
            Attribute::new("data-click", action, ()).hash(&mut node);
        }

        if let Some(on_input) = &self.tag.on_input {
            // TODO: avoid double allocation (again in render)
            let action = format!("{}::{}", on_input.module, on_input.name);
            Attribute::new("data-input", action, ()).hash(&mut node);
        }

        self.tag.attrs.hash(&mut node);
        let content = self.content.render(node.content());
        let hash = node.end();
        hash_tree.changed_or_else(hash, || HtmlTagRenderer {
            tag: self.tag,
            content,
        })
    }
}

pub struct HtmlTagRenderer<S, A, R> {
    tag: HtmlTag<S, A>,
    content: Option<R>,
}

impl<S, A, R> Render for HtmlTagRenderer<S, A, R>
where
    A: Attributes,
    R: Render,
{
    fn render(self, mut out: impl Write, is_update: bool) -> fmt::Result {
        write!(&mut out, "<{}", self.tag.tag)?;

        if let Some(on_click) = self.tag.on_click {
            // TODO: avoid allocation
            let action = format!("{}::{}", on_click.module, on_click.name);
            Attribute::new("data-click", action, ()).render(&mut out)?;
        }

        if let Some(on_input) = self.tag.on_input {
            // TODO: avoid allocation
            let action = format!("{}::{}", on_input.module, on_input.name);
            Attribute::new("data-input", action, ()).render(&mut out)?;
        }

        self.tag.attrs.render(&mut out)?;

        // Handle void elements. Content is simply ignored.
        if is_void_element(self.tag.tag) {
            write!(&mut out, "/>")?;
            return Ok(());
        }

        write!(&mut out, ">")?;

        self.content.render(&mut out, is_update)?;

        write!(&mut out, "</{}>", self.tag.tag)?;
        Ok(())
    }
}

fn is_void_element(tag: &str) -> bool {
    // https://html.spec.whatwg.org/multipage/syntax.html#elements-2
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
