mod marker;
#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Neg;

use serde::Deserialize;
use serde_json::value::RawValue;
use twox_hash::XxHash32;

use self::marker::Marker;
pub use self::marker::ViewHashTree;
use crate::View;

pub struct Renderer {
    out: String,
    hasher: XxHash32,
}

pub(crate) struct Out {
    pub view: String,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Renderer {
            out: String::with_capacity(256),
            hasher: XxHash32::default(),
        }
    }

    pub(crate) fn end(mut self) -> Out {
        Out { view: self.out }
    }

    pub fn element(mut self, tag: &'static str) -> Result<ElementRenderer, crate::Error> {
        let parent_hasher = std::mem::take(&mut self.hasher);
        self.write(tag.as_bytes());
        let offset = self.out.len();
        write!(&mut self.out, "<{tag}").map_err(crate::error::InternalError::from)?;
        Ok(ElementRenderer {
            tag,
            offset,
            parent_hasher,
            renderer: self,
            content_started: false,
        })
    }

    pub fn text(mut self) -> TextRenderer {
        TextRenderer {
            hasher: Default::default(),
            previous_len: self.out.len(),
            renderer: self,
        }
    }

    /// Adds a component to the tree and returns whether it is a new component.
    pub fn component(mut self, instance_id: u32) -> ComponentRenderer {
        ComponentRenderer {
            hasher: XxHash32::default(),
            previous_len: self.out.len(),
            renderer: self,
            instance_id,
        }
    }
}

pub struct ElementRenderer {
    tag: &'static str,
    offset: usize,
    renderer: Renderer,
    parent_hasher: XxHash32,
    content_started: bool,
}

impl ElementRenderer {
    pub fn attribute(&mut self, name: &str, value: &str) -> Result<(), fmt::Error> {
        if self.content_started {
            todo!("throw error: content started");
        }
        self.renderer.write(name.as_bytes());
        self.renderer.write(value.as_bytes());
        write!(
            &mut self.renderer.out,
            r#" {}="{}""#,
            name, // TODO: validate/escape attr name
            escape_attribute_value(value)
        )
    }

    pub async fn content(mut self, view: impl View) -> Result<Renderer, crate::Error> {
        if is_void_element(self.tag) {
            todo!("throw error: void tags cannot have content");
        }
        if !self.content_started {
            self.content_started = true;
            write!(&mut self.renderer.out, ">").map_err(crate::error::InternalError::from)?;
        }

        self.renderer = view.render(self.renderer).await?;
        self.end()
    }

    pub fn end(mut self) -> Result<Renderer, crate::Error> {
        if !self.content_started && !is_void_element(self.tag) {
            write!(&mut self.renderer.out, ">").map_err(crate::error::InternalError::from)?;
        }

        let hash = self.renderer.finish() as u32;
        self.parent_hasher.write_u32(hash);
        std::mem::swap(&mut self.renderer.hasher, &mut self.parent_hasher);

        // if self.renderer.changed(hash, self.offset)? {
        // Handle void elements. Content is simply ignored.
        if is_void_element(self.tag) {
            write!(&mut self.renderer.out, "/>").map_err(crate::error::InternalError::from)?;
        } else {
            write!(&mut self.renderer.out, "</{}>", self.tag)
                .map_err(crate::error::InternalError::from)?;
        }
        // }

        Ok(self.renderer)
    }
}

pub struct TextRenderer {
    hasher: XxHash32,
    renderer: Renderer,
    previous_len: usize,
}

impl TextRenderer {
    pub fn end(mut self) -> Result<Renderer, crate::Error> {
        let hash = self.hasher.finish() as u32;
        self.renderer.write_u32(hash);

        Ok(self.renderer)
    }
}

pub struct ComponentRenderer {
    hasher: XxHash32,
    renderer: Renderer,
    previous_len: usize,
    instance_id: u32,
}

impl ComponentRenderer {
    pub async fn content(
        mut self,
        view: impl View,
        previous_tree: Option<ViewHashTree>,
    ) -> Result<Renderer, crate::Error> {
        let r = Renderer {
            out: mem::take(&mut self.renderer.out),
            hasher: self.hasher,
        };
        let mut r = view.render(r).await?;

        // Ensure component is detected as changed if its instance id changed
        r.write_u32(self.instance_id);

        let hash = r.finish() as u32;

        // Restore parent renderer
        self.renderer.out = mem::take(&mut r.out);

        // Add component to parent renderer
        self.renderer.write_u32(hash);

        // Write a random hash to ensure the ascendents of a changed compontent are always
        // invalidated
        // TODO: anyway around that?
        // #[cfg(not(test))]
        // if changed {
        //     self.renderer.write_u32(rand::random());
        // }

        Ok(self.renderer)
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

impl Hasher for Renderer {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes);
    }
}

impl Write for TextRenderer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.hasher.write(s.as_bytes());
        self.renderer.out.write_str(s)
    }
}

impl Hasher for ComponentRenderer {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes);
    }
}

impl Write for ComponentRenderer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.renderer.write(s.as_bytes());
        self.renderer.out.write_str(s)
    }
}

impl Write for Renderer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.out.write_str(s)
    }
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

pub fn is_void_element(tag: &str) -> bool {
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
