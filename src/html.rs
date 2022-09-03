use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::hash::Hasher;
use std::ops::Deref;

use serde::{Deserialize, Serialize};
use twox_hash::XxHash32;

use crate::view::{View, ViewHash};

pub fn div<A>() -> HtmlTagBuilder<A> {
    HtmlTagBuilder {
        tag: "div",
        ..Default::default()
    }
}

pub fn button<A>() -> HtmlTagBuilder<A> {
    HtmlTagBuilder {
        tag: "button",
        ..Default::default()
    }
}

pub fn custom<A>(tag: &'static str) -> HtmlTagBuilder<A> {
    HtmlTagBuilder {
        tag,
        ..Default::default()
    }
}

pub struct HtmlTag<V, A> {
    builder: HtmlTagBuilder<A>,
    content: V,
}

pub struct HtmlTagBuilder<A = ()> {
    tag: &'static str,
    attrs: Option<HashMap<&'static str, Cow<'static, str>>>,
    on_click: Option<A>,
    on_input: Option<A>,
}

#[non_exhaustive]
pub struct InputEvent {
    pub value: InputValue,
}

pub struct InputValue(Cow<'static, str>);

impl<A> HtmlTagBuilder<A> {
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
    pub fn on_click(mut self, action: A) -> HtmlTagBuilder<A> {
        self.on_click = Some(action);
        self
    }

    // TODO: not available for all tags (e.g. only for inputs)
    pub fn on_input(mut self, action: impl FnOnce(InputEvent) -> A) -> HtmlTagBuilder<A> {
        self.on_input = Some(action(InputEvent {
            value: InputValue("".into()),
        }));
        self
    }

    pub fn content<V: View<A>>(self, content: V) -> HtmlTag<V, A> {
        HtmlTag {
            builder: self,
            content,
        }
    }
}

impl<V, A> View<A> for HtmlTag<V, A>
where
    V: View<A>,
    A: Serialize,
{
    fn render(mut self, mut out: impl Write) -> Result<ViewHash, fmt::Error> {
        let mut hasher = XxHash32::default();
        hasher.write(self.builder.tag.as_bytes());

        write!(&mut out, "<{}", self.builder.tag)?;

        if let Some(on_click) = self.builder.on_click.take() {
            // TODO: unwrap
            let action = serde_json::to_string(&on_click).unwrap();
            hasher.write(b"on_click");
            hasher.write(action.as_bytes());
            self.builder = self.builder.attr("data-click", action);
        }

        if let Some(on_input) = self.builder.on_input.take() {
            // TODO: unwrap
            let action = serde_json::to_string(&on_input).unwrap();
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

impl<A> View<A> for HtmlTagBuilder<A>
where
    A: Serialize,
{
    fn render(self, out: impl Write) -> Result<ViewHash, fmt::Error> {
        HtmlTag {
            builder: self,
            content: (),
        }
        .render(out)
    }
}

impl<A> Default for HtmlTagBuilder<A> {
    fn default() -> Self {
        Self {
            tag: "div",
            attrs: None,
            on_click: None,
            on_input: None,
        }
    }
}

impl Serialize for InputValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("_##InputValue")
    }
}

impl<'de> Deserialize<'de> for InputValue {
    fn deserialize<D>(deserializer: D) -> Result<InputValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Cow<'static, str> = Deserialize::deserialize(deserializer)?;
        Ok(InputValue(value))
    }
}

impl InputValue {
    pub fn take(self) -> Cow<'static, str> {
        self.0
    }
}

impl Deref for InputValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
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
