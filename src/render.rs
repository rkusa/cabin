#[cfg(test)]
mod tests;

use std::fmt::{self, Display, Write};
use std::hash::{Hash, Hasher};

use twox_hash::XxHash32;

use crate::View;

pub struct Renderer {
    out: String,
    hasher: XxHash32,
    skip_hash: bool,
}

pub(crate) struct Out {
    pub view: String,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Renderer {
            out: String::with_capacity(256),
            hasher: XxHash32::default(),
            skip_hash: false,
        }
    }

    pub(crate) fn end(self) -> Out {
        Out { view: self.out }
    }

    pub fn element(
        mut self,
        tag: &'static str,
        include_hash: bool,
    ) -> Result<ElementRenderer, crate::Error> {
        let parent_hasher = std::mem::take(&mut self.hasher);
        self.write(tag.as_bytes());

        let should_write_id =
            include_hash && !matches!(tag, "html" | "body" | "head") && !self.skip_hash;
        let parent_skip_hash = self.skip_hash;
        if matches!(tag, "head") {
            self.skip_hash = true;
        }

        write!(&mut self.out, "<{tag}").map_err(crate::error::InternalError::from)?;

        let hash_offset = if should_write_id {
            write!(&mut self.out, " hash=\"").map_err(crate::error::InternalError::from)?;
            let hash_offset = self.out.len();
            // Write placeholder id which will be replaced later on
            write!(&mut self.out, "00000000\"").map_err(crate::error::InternalError::from)?;
            Some(hash_offset)
        } else {
            None
        };

        Ok(ElementRenderer {
            tag,
            parent_hasher,
            parent_skip_hash,
            renderer: self,
            content_started: false,
            hash_offset,
        })
    }

    pub fn text(self) -> TextRenderer {
        TextRenderer {
            hasher: Default::default(),
            renderer: self,
        }
    }
}

pub struct ElementRenderer {
    tag: &'static str,
    renderer: Renderer,
    parent_hasher: XxHash32,
    parent_skip_hash: bool,
    content_started: bool,
    hash_offset: Option<usize>,
}

impl ElementRenderer {
    pub fn attribute(&mut self, name: &str, value: impl Display + Hash) -> Result<(), fmt::Error> {
        if self.content_started {
            todo!("throw error: content started");
        }
        self.renderer.write(name.as_bytes());
        value.hash(&mut self.renderer);

        write!(&mut self.renderer.out, r#" {}=""#, name,)?;
        write!(Escape::attribute_value(&mut self.renderer.out), "{}", value)?;
        write!(&mut self.renderer.out, r#"""#)?;

        Ok(())
    }

    pub fn empty_attribute(&mut self, name: &str) -> Result<(), fmt::Error> {
        if self.content_started {
            todo!("throw error: content started");
        }
        self.renderer.write(name.as_bytes());

        write!(&mut self.renderer.out, r#" {}"#, name,)?;

        Ok(())
    }

    pub async fn content(mut self, view: impl View) -> Result<Renderer, crate::Error> {
        if !self.content_started {
            self.content_started = true;
            write!(&mut self.renderer.out, ">").map_err(crate::error::InternalError::from)?;
        }

        self.renderer = view.render(self.renderer, false).await?;
        self.end(false)
    }

    pub fn end(mut self, is_void_element: bool) -> Result<Renderer, crate::Error> {
        if !self.content_started && !is_void_element {
            write!(&mut self.renderer.out, ">").map_err(crate::error::InternalError::from)?;
        }

        let hash = self.renderer.finish() as u32;
        if let Some(offset) = self.hash_offset {
            // FIXME: would be better to directly write to the specified location instead of the
            // additional string allocation
            self.renderer
                .out
                .replace_range(offset..offset + 8, &format!("{:x}", hash));
        }

        self.parent_hasher.write_u32(hash);
        std::mem::swap(&mut self.renderer.hasher, &mut self.parent_hasher);

        // if self.renderer.changed(hash, self.offset)? {
        // Handle void elements. Content is simply ignored.
        if is_void_element {
            write!(&mut self.renderer.out, "/>").map_err(crate::error::InternalError::from)?;
        } else {
            write!(&mut self.renderer.out, "</{}>", self.tag)
                .map_err(crate::error::InternalError::from)?;
        }
        // }

        self.renderer.skip_hash = self.parent_skip_hash;
        Ok(self.renderer)
    }
}

pub struct TextRenderer {
    hasher: XxHash32,
    renderer: Renderer,
}

impl TextRenderer {
    pub fn end(mut self) -> Result<Renderer, crate::Error> {
        let hash = self.hasher.finish() as u32;
        self.renderer.write_u32(hash);

        Ok(self.renderer)
    }
}

pub struct Escape<W> {
    wr: W,
    escape_fn: fn(char) -> Option<&'static str>,
}

impl<W> Escape<W> {
    pub fn attribute_value(wr: W) -> Self {
        Escape {
            wr,
            escape_fn: escape_attribute_value_char,
        }
    }

    pub fn content(wr: W) -> Self {
        Escape {
            wr,
            escape_fn: escape_content_char,
        }
    }
}

impl<W> fmt::Write for Escape<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut replacements = s
            .char_indices()
            .filter_map(|(i, ch)| (self.escape_fn)(ch).map(|s| (i, s)))
            .peekable();
        if replacements.peek().is_none() {
            return self.wr.write_str(s);
        }

        let mut pos = 0;
        for (i, sub) in replacements {
            if i > pos {
                self.wr.write_str(&s[pos..i])?;
            }
            self.wr.write_str(sub)?;
            pos = i + 1;
        }
        if pos < s.len() {
            self.wr.write_str(&s[pos..s.len()])?;
        }

        Ok(())
    }
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

impl Write for Renderer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.out.write_str(s)
    }
}

fn escape_attribute_value_char(ch: char) -> Option<&'static str> {
    // Not escaping ' -> because cabin always warps attribute values in double-quotes
    match ch {
        '"' => Some("&quot;"),
        '&' => Some("&amp;"),
        _ => None,
    }
}

fn escape_content_char(ch: char) -> Option<&'static str> {
    match ch {
        '<' => Some("&lt;"),
        '&' => Some("&amp;"),
        _ => None,
    }
}
