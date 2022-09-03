use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Write};

use serde::Serialize;

use crate::view::View;

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
    content: Option<V>,
}

pub struct HtmlTagBuilder<A = ()> {
    tag: &'static str,
    attrs: Option<HashMap<&'static str, Cow<'static, str>>>,
    on_click: Option<A>,
}

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

    pub fn content<V: View<A>>(self, content: V) -> HtmlTag<V, A> {
        HtmlTag {
            builder: self,
            content: Some(content),
        }
    }
}

impl<V, A> View<A> for HtmlTag<V, A>
where
    V: View<A>,
    A: Serialize,
{
    fn render(mut self, mut out: impl Write) -> fmt::Result {
        write!(&mut out, "<{}", self.builder.tag)?;
        if let Some(on_click) = self.builder.on_click.take() {
            // TODO: unwrap
            let action = serde_json::to_string(&on_click).unwrap();
            self.builder = self.builder.attr("data-click", action);
        }

        if let Some(attrs) = self.builder.attrs {
            for (name, value) in attrs {
                write!(
                    &mut out,
                    r#" {}="{}""#,
                    name, // TODO: validate/escape attr name
                    escape_attribute_value(&value)
                )?;
            }
        }

        if let Some(content) = self.content {
            write!(&mut out, ">")?;
            content.render(&mut out)?;
            write!(&mut out, "</{}>", self.builder.tag)?;
        } else if !matches!(self.builder.tag, "script") {
            write!(&mut out, "/>")?;
        } else {
            write!(&mut out, "></{}>", self.builder.tag)?;
        }

        Ok(())
    }
}

impl<A> View<A> for HtmlTagBuilder<A>
where
    A: Serialize,
{
    fn render(self, out: impl Write) -> fmt::Result {
        HtmlTag {
            builder: self,
            content: None::<()>,
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
