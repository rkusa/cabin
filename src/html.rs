use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::hash::Hasher;

use serde::Deserialize;
use twox_hash::XxHash32;

use crate::action::EventAction;
use crate::view::{View, ViewHash};
use crate::Action;

pub fn div<S>() -> HtmlTagBuilder<S> {
    HtmlTagBuilder {
        tag: "div",
        ..Default::default()
    }
}

pub fn button<S>() -> HtmlTagBuilder<S> {
    HtmlTagBuilder {
        tag: "button",
        ..Default::default()
    }
}

pub fn custom<S>(tag: &'static str) -> HtmlTagBuilder<S> {
    HtmlTagBuilder {
        tag,
        ..Default::default()
    }
}

pub struct HtmlTag<V, S> {
    builder: HtmlTagBuilder<S>,
    content: V,
}

pub struct HtmlTagBuilder<S = ()> {
    tag: &'static str,
    attrs: Option<HashMap<&'static str, Cow<'static, str>>>,
    on_click: Option<Action<S>>,
    on_input: Option<EventAction<S, InputEvent>>,
}

#[derive(Deserialize)]
#[non_exhaustive]
pub struct InputEvent {
    pub value: String,
}

impl<S> HtmlTagBuilder<S> {
    pub(crate) fn new(tag: &'static str) -> Self {
        HtmlTagBuilder {
            tag,
            ..Default::default()
        }
    }

    pub fn attr(mut self, name: &'static str, value: impl Into<Cow<'static, str>>) -> Self {
        let value = value.into();
        let mut attrs = self.attrs.take().unwrap_or_default();
        attrs.insert(name, value);
        self.attrs = Some(attrs);
        self
    }

    // TODO: not available for all tags (e.g. only for buttons)
    pub fn on_click(mut self, action: Action<S>) -> HtmlTagBuilder<S> {
        self.on_click = Some(action);
        self
    }

    // TODO: not available for all tags (e.g. only for inputs)
    pub fn on_input(mut self, action: EventAction<S, InputEvent>) -> HtmlTagBuilder<S> {
        self.on_input = Some(action);
        self
    }

    pub fn content<V: View<S>>(self, content: V) -> HtmlTag<V, S> {
        HtmlTag {
            builder: self,
            content,
        }
    }
}

impl<V, S> View<S> for HtmlTag<V, S>
where
    V: View<S>,
{
    fn render(mut self, mut out: impl Write) -> Result<ViewHash, fmt::Error> {
        let mut hasher = XxHash32::default();
        hasher.write(self.builder.tag.as_bytes());

        write!(&mut out, "<{}", self.builder.tag)?;

        if let Some(on_click) = self.builder.on_click.take() {
            // TODO: unwrap
            let action = on_click.id;
            hasher.write(b"on_click");
            hasher.write(action.as_bytes());
            self.builder = self.builder.attr("data-click", action);
        }

        if let Some(on_input) = self.builder.on_input.take() {
            // TODO: unwrap
            let action = on_input.id;
            hasher.write(b"on_input");
            hasher.write(action.as_bytes());
            self.builder = self.builder.attr("data-input", action);
        }

        if let Some(attrs) = self.builder.attrs {
            for (name, value) in attrs {
                hasher.write(name.as_bytes());
                hasher.write(value.as_bytes());

                write!(
                    &mut out,
                    r#" {}="{}""#,
                    name, // TODO: validate/escape attr name
                    escape_attribute_value(&value)
                )?;
            }
        }

        let mut inner = String::new();
        let child_hash = self.content.render(&mut inner)?;
        hasher.write_u32(child_hash.hash());

        let hash = hasher.finish() as u32;
        write!(&mut out, r#" data-hash="{}""#, hash)?;

        if !inner.is_empty() {
            write!(&mut out, ">{}</{}>", inner, self.builder.tag)?;
            Ok(child_hash.into_parent(hash))
        } else if matches!(self.builder.tag, "input") {
            write!(&mut out, "/>")?;
            Ok(ViewHash::Leaf(hash))
        } else {
            write!(&mut out, "></{}>", self.builder.tag)?;
            Ok(ViewHash::Leaf(hash))
        }
    }
}

impl<S> View<S> for HtmlTagBuilder<S> {
    fn render(self, out: impl Write) -> Result<ViewHash, fmt::Error> {
        HtmlTag {
            builder: self,
            content: (),
        }
        .render(out)
    }
}

impl<S> Default for HtmlTagBuilder<S> {
    fn default() -> Self {
        Self {
            tag: "div",
            attrs: None,
            on_click: None,
            on_input: None,
        }
    }
}

pub fn escape_attribute_value(input: &str) -> Cow<str> {
    let mut replacements = input
        .char_indices()
        .filter_map(|(i, ch)| escape_attribute_value_char(ch).map(|s| (i, s)))
        .peekable();
    if replacements.peek().is_none() {
        return Cow::Borrowed(input);
    }

    let mut escaped = String::with_capacity(input.len());
    let mut pos = 0;
    for (i, sub) in replacements {
        if i > pos {
            escaped.push_str(&input[pos..i]);
        }
        escaped.push_str(sub);
        pos = i + 1;
    }
    if pos < input.len() {
        escaped.push_str(&input[pos..input.len()]);
    }

    Cow::Owned(escaped)
}

fn escape_attribute_value_char(ch: char) -> Option<&'static str> {
    match ch {
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '\'' => Some("&apos;"),
        '&' => Some("&amp;"),
        '"' => Some("&quot;"),
        _ => None,
    }
}
