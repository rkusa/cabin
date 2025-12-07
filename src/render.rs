use std::fmt::{self, Display, Write};
use std::hash::{Hash, Hasher};

use http::{HeaderMap, HeaderValue};
use twox_hash::XxHash32;

use crate::Context;
use crate::error::InternalError;
use crate::event::Event;
use crate::html::events::CustomEvent;

const DEFAULT_CAPACITY: usize = 256;

pub struct Renderer {
    out: String,
    headers: HeaderMap<HeaderValue>,
    hasher: XxHash32,
    is_update: bool,
}

pub struct Out {
    pub html: String,
    pub headers: HeaderMap<HeaderValue>,
}

impl Renderer {
    pub fn new(is_update: bool) -> Self {
        Self {
            out: String::with_capacity(DEFAULT_CAPACITY),
            headers: Default::default(),
            hasher: XxHash32::default(),
            is_update,
        }
    }

    pub fn reset(&mut self) {
        self.out.clear();
        self.headers.clear();
        self.hasher = XxHash32::default();
    }

    pub fn append(&mut self, mut other: Renderer) {
        if self.out.is_empty() {
            std::mem::swap(&mut self.out, &mut other.out);
        } else {
            self.out.push_str(&other.out);
        }
        self.headers.extend(std::mem::take(&mut other.headers));
        let hash = other.hasher.finish() as u32;
        self.hasher.write_u32(hash);

        Context::release_renderer_into_task(other);
    }

    pub fn end(self) -> Out {
        Out {
            html: self.out,
            headers: self.headers,
        }
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.headers
    }

    pub fn text<'r>(&'r mut self) -> TextRenderer<'r> {
        TextRenderer {
            hasher: Default::default(),
            renderer: self,
        }
    }

    pub fn attribute(&mut self, name: &str, value: impl Display + Hash) {
        self.hasher.write(name.as_bytes());
        value.hash(&mut self.hasher);

        write!(&mut self.out, r#" {name}=""#).unwrap();
        write!(Escape::attribute_value(&mut self.out), "{value}").unwrap();
        write!(&mut self.out, r#"""#).unwrap();
    }

    pub fn empty_attribute(&mut self, name: &str) {
        self.hasher.write(name.as_bytes());
        write!(&mut self.out, r#" {name}"#).unwrap();
    }

    pub fn event_attributes<E: serde::Serialize + Event>(
        &mut self,
        event: CustomEvent<E>,
    ) -> Result<(), crate::Error> {
        // event id
        {
            let pos_name = self.out.len();
            write!(&mut self.out, " cabin-{}", event.name).unwrap();
            self.out[(pos_name + 1)..].hash(&mut self.hasher);
            write!(&mut self.out, r#"=""#).unwrap();

            let pos_value = self.out.len();
            write!(&mut self.out, "{}", E::ID).unwrap();
            self.out[pos_value..].hash(&mut self.hasher);
            write!(&mut self.out, r#"""#).unwrap();
        }

        // event payload
        {
            let pos_name = self.out.len();
            write!(&mut self.out, " cabin-{}-payload", event.name).unwrap();
            self.out[(pos_name + 1)..].hash(&mut self.hasher);
            write!(&mut self.out, r#"=""#).unwrap();

            let pos_value = self.out.len();
            serde_json::to_writer(Escape::attribute_value(&mut self.out), &event.event).map_err(
                |err| InternalError::Serialize {
                    what: format!("{} event", event.name).into(),
                    err,
                },
            )?;
            self.out[pos_value..].hash(&mut self.hasher);
            write!(&mut self.out, r#"""#).unwrap();
        }

        Ok(())
    }

    pub fn start_element(&mut self, tag: &'static str) -> Option<usize> {
        self.hasher.write(tag.as_bytes());

        write!(&mut self.out, "<{tag}").unwrap();

        let should_write_hash = !matches!(tag, "html" | "body" | "head" | "option");
        if should_write_hash {
            write!(&mut self.out, " hash=\"").unwrap();
            let hash_offset = self.out.len();
            // Write placeholder id which will be replaced later on
            write!(&mut self.out, "00000000\"").unwrap();
            Some(hash_offset)
        } else {
            None
        }
    }

    pub fn start_content(&mut self) {
        write!(&mut self.out, ">").unwrap();
    }

    pub fn end_element(
        &mut self,
        tag: &'static str,
        is_void_element: bool,
        hash_offset: Option<usize>,
    ) {
        let hash = self.hasher.finish() as u32;
        if let Some(offset) = hash_offset {
            write!(WriteInto::new(&mut self.out, offset), "{hash:x}").unwrap();
        }

        // if self.renderer.changed(hash, self.offset)? {
        // Handle void elements. Content is simply ignored.
        if is_void_element {
            write!(&mut self.out, "/>").unwrap();
        } else {
            write!(&mut self.out, "</{tag}>").unwrap();
        }
        // }
    }

    pub fn take_hasher(&mut self) -> XxHash32 {
        std::mem::replace(&mut self.hasher, Default::default())
    }

    pub fn merge_hasher(&mut self, mut hasher: XxHash32) {
        let hash = self.hasher.finish() as u32;
        hasher.write_u32(hash);
        self.hasher = hasher;
    }
}

pub struct TextRenderer<'r> {
    hasher: XxHash32,
    renderer: &'r mut Renderer,
}

impl<'r> TextRenderer<'r> {
    pub fn end(self) {
        let hash = self.hasher.finish() as u32;
        self.renderer.hasher.write_u32(hash);
    }
}

pub struct Escape<W> {
    wr: W,
    escape_fn: fn(char, Option<char>) -> Option<&'static str>,
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

    pub fn script(wr: W) -> Self {
        Escape {
            wr,
            escape_fn: escape_script_char,
        }
    }
}

impl<W> fmt::Write for Escape<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut next = s.chars();
        next.next();
        let mut replacements = s
            .char_indices()
            .filter_map(|(i, ch)| (self.escape_fn)(ch, next.next()).map(|s| (i, s)))
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

impl<W> std::io::Write for Escape<W>
where
    W: fmt::Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf).map_err(std::io::Error::other)?;
        self.write_str(s).map_err(std::io::Error::other)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct WriteInto<'a> {
    out: &'a mut String,
    offset: usize,
}

impl<'a> WriteInto<'a> {
    pub fn new(out: &'a mut String, offset: usize) -> Self {
        Self { out, offset }
    }
}

impl fmt::Write for WriteInto<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.out
            .replace_range(self.offset..self.offset + s.len(), s);
        self.offset += s.len();

        Ok(())
    }
}

impl<'r> Write for TextRenderer<'r> {
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

fn escape_attribute_value_char(ch: char, _next: Option<char>) -> Option<&'static str> {
    // Not escaping ' -> because cabin always warps attribute values in double-quotes
    match ch {
        '"' => Some("&quot;"),
        '&' => Some("&amp;"),
        _ => None,
    }
}

fn escape_content_char(ch: char, _next: Option<char>) -> Option<&'static str> {
    match ch {
        '<' => Some("&lt;"),
        '&' => Some("&amp;"),
        _ => None,
    }
}

fn escape_script_char(ch: char, next: Option<char>) -> Option<&'static str> {
    match (ch, next) {
        ('<', Some('/')) => Some("<\\"),
        ('<', Some('!')) => Some("<\\"),
        _ => None,
    }
}
