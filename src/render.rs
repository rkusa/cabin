use std::collections::HashMap;
use std::fmt::{self, Display, Write};
use std::hash::{Hash, Hasher};

use http::{HeaderMap, HeaderValue};
use twox_hash::XxHash32;

use crate::View;
use crate::error::InternalError;
use crate::event::Event;
use crate::html::events::CustomEvent;
use crate::scope::Scope;
use crate::style::{ClassName, StyleDefinition};
use crate::view::RenderFuture;

// This covers about 75% of [Renderer] usages in my largest app as per 2025-12-10.
const DEFAULT_CAPACITY: usize = 128;

pub struct Renderer {
    out: String,
    headers: HeaderMap<HeaderValue>,
    styles: HashMap<ClassName, StyleDefinition>,
    hasher: XxHash32,
    is_update: bool,
    disable_hashes: bool,
}

pub struct Out {
    pub html: String,
    pub headers: HeaderMap<HeaderValue>,
}

impl Renderer {
    pub fn new(is_update: bool, disable_hashes: bool) -> Self {
        Self {
            out: String::with_capacity(DEFAULT_CAPACITY),
            headers: Default::default(),
            // Will stay empty for about 40% of renderers, and will grow to high numbers for about
            // 15%. Thus decided to keep them empty by default and not reserve any
            // capacity.
            styles: Default::default(),
            hasher: XxHash32::default(),
            disable_hashes,
            is_update,
        }
    }

    pub fn reset(&mut self) {
        self.out.truncate(0);
        self.headers.clear();
        self.styles.clear();
        self.hasher = XxHash32::default();
    }

    pub fn append(&mut self, mut other: Renderer) {
        if self.out.is_empty() {
            std::mem::swap(&mut self.out, &mut other.out);
        } else if other.out.capacity() > self.out.capacity() {
            other.out.insert_str(0, &self.out);
            std::mem::swap(&mut self.out, &mut other.out);
        } else {
            self.out += &other.out;
        }
        if self.headers.is_empty() {
            std::mem::swap(&mut self.headers, &mut other.headers);
        } else if other.headers.capacity() > self.headers.capacity() {
            other.headers.extend(self.headers.drain());
            std::mem::swap(&mut self.headers, &mut other.headers);
        } else {
            self.headers.extend(other.headers.drain());
        }
        if self.styles.is_empty() {
            std::mem::swap(&mut self.styles, &mut other.styles);
        } else if other.styles.capacity() > self.styles.capacity() {
            other.styles.extend(self.styles.drain());
            std::mem::swap(&mut self.styles, &mut other.styles);
        } else {
            self.styles.extend(other.styles.drain());
        }
        let hash = other.hasher.finish() as u32;
        self.hasher.write_u32(hash);
        Scope::release_renderer_to_task(other);
    }

    // FIXME: remove unnecessary error
    pub fn end(self) -> Result<Out, crate::Error> {
        Ok(Out {
            html: self.out,
            headers: self.headers,
        })
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }

    pub fn element(mut self, tag: &'static str) -> ElementRenderer {
        let parent_hasher = std::mem::take(&mut self.hasher);
        self.hasher.write(tag.as_bytes());

        write!(&mut self.out, "<{tag}").unwrap();

        let should_write_hash =
            !matches!(tag, "body" | "head" | "html" | "link" | "meta" | "option");
        let hash_offset = if should_write_hash && !self.disable_hashes {
            write!(&mut self.out, " hash=\"").unwrap();
            let hash_offset = self.out.len();
            // Write placeholder id which will be replaced later on
            write!(&mut self.out, "00000000\"").unwrap();
            Some(hash_offset)
        } else {
            None
        };

        ElementRenderer {
            tag,
            parent_hasher,
            renderer: self,
            content_started: false,
            hash_offset,
        }
    }

    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.headers
    }

    pub fn text(self) -> TextRenderer {
        TextRenderer {
            hasher: Default::default(),
            renderer: self,
        }
    }

    pub fn append_style(&mut self, style: StyleDefinition) -> ClassName {
        let class_name = style.class_name();
        self.styles.insert(class_name, style);
        class_name
    }

    pub(crate) fn build_styles(&mut self, include_base: bool) -> String {
        let other: [&str; _] = [
            #[cfg(not(test))]
            include_str!("./style/base.css"),
            // preflight
            #[cfg(all(feature = "preflight", not(test)))]
            "@layer preflight {\n",
            #[cfg(all(feature = "preflight", not(test)))]
            include_str!("./style/preflight/preflight-v4.1.18.css"),
            #[cfg(all(feature = "preflight", not(test)))]
            "}\n",
            // forms
            #[cfg(all(feature = "forms", not(test)))]
            "@layer forms {\n",
            #[cfg(all(feature = "forms", not(test)))]
            include_str!("./style/forms/forms-v0.5.11.css"),
            #[cfg(all(feature = "forms", not(test)))]
            "}\n",
        ];

        let mut styles = self.styles.drain().map(|(_, v)| v).collect::<Vec<_>>();
        styles.sort_by(|a, b| a.modifier.cmp(&b.modifier));

        let cap: usize = if include_base {
            other.iter().map(|s| s.len()).sum::<usize>()
        } else {
            0
        } + self.styles.len() * 128;
        let mut css = String::with_capacity(cap);
        if include_base {
            for s in other {
                css += s;
            }
        }

        for s in styles {
            s.write_to(&mut css);
        }

        css
    }
}

pub struct ElementRenderer {
    tag: &'static str,
    pub(crate) renderer: Renderer,
    parent_hasher: XxHash32,
    content_started: bool,
    hash_offset: Option<usize>,
}

impl ElementRenderer {
    pub fn attribute(&mut self, name: &str, value: impl Display + Hash) {
        if self.content_started {
            todo!("throw error: content started");
        }
        self.renderer.hasher.write(name.as_bytes());
        value.hash(&mut self.renderer.hasher);

        write!(&mut self.renderer, r#" {name}=""#).unwrap();
        Write::write_fmt(
            &mut Escape::attribute_value(&mut self.renderer),
            format_args!("{value}"),
        )
        .unwrap();
        write!(&mut self.renderer, r#"""#).unwrap();
    }

    pub fn empty_attribute(&mut self, name: &str) {
        if self.content_started {
            todo!("throw error: content started");
        }
        self.renderer.hasher.write(name.as_bytes());

        write!(&mut self.renderer, r#" {name}"#).unwrap();
    }

    pub fn event_attributes<E: serde::Serialize + Event>(
        &mut self,
        event: CustomEvent<E>,
    ) -> Result<(), crate::Error> {
        // event id
        {
            let pos_name = self.renderer.out.len();
            write!(&mut self.renderer, " cabin-{}", event.name).unwrap();
            self.renderer.out[(pos_name + 1)..].hash(&mut self.renderer.hasher);
            write!(&mut self.renderer, r#"=""#).unwrap();

            let pos_value = self.renderer.out.len();
            write!(&mut self.renderer, "{}", E::ID).unwrap();
            self.renderer.out[pos_value..].hash(&mut self.renderer.hasher);
            write!(&mut self.renderer, r#"""#).unwrap();
        }

        // event payload
        {
            let pos_name = self.renderer.out.len();
            write!(&mut self.renderer, " cabin-{}-payload", event.name).unwrap();
            self.renderer.out[(pos_name + 1)..].hash(&mut self.renderer.hasher);
            write!(&mut self.renderer, r#"=""#).unwrap();

            let pos_value = self.renderer.out.len();
            serde_json::to_writer(Escape::attribute_value(&mut self.renderer), &event.event)
                .map_err(|err| InternalError::Serialize {
                    what: format!("{} event", event.name).into(),
                    err,
                })?;
            self.renderer.out[pos_value..].hash(&mut self.renderer.hasher);
            write!(&mut self.renderer, r#"""#).unwrap();
        }

        Ok(())
    }

    pub fn content(self, view: impl View) -> RenderFuture {
        let ElementRenderer {
            tag,
            mut renderer,
            parent_hasher,
            mut content_started,
            hash_offset,
        } = self;

        if !content_started {
            content_started = true;
            write!(&mut renderer.out, ">").unwrap();
        }

        match view.render(renderer) {
            RenderFuture::Ready(Ok(renderer)) => RenderFuture::Ready(Ok(ElementRenderer {
                tag,
                renderer,
                parent_hasher,
                content_started,
                hash_offset,
            }
            .end(false))),
            RenderFuture::Ready(Err(err)) => RenderFuture::Ready(Err(err)),
            RenderFuture::Future(fut) => RenderFuture::Future(Box::pin(async move {
                Ok(ElementRenderer {
                    tag,
                    renderer: fut.await?,
                    parent_hasher,
                    content_started,
                    hash_offset,
                }
                .end(false))
            })),
        }
    }

    pub fn end(mut self, is_void_element: bool) -> Renderer {
        if !self.content_started && !is_void_element {
            write!(&mut self.renderer, ">").unwrap();
        }

        let hash = self.renderer.hasher.finish() as u32;
        if let Some(offset) = self.hash_offset {
            write!(WriteInto::new(&mut self.renderer.out, offset), "{hash:x}").unwrap();
        }

        self.parent_hasher.write_u32(hash);
        std::mem::swap(&mut self.renderer.hasher, &mut self.parent_hasher);
        // Handle void elements. Content is simply ignored.
        if is_void_element {
            write!(&mut self.renderer, "/>").unwrap();
        } else {
            write!(&mut self.renderer, "</{}>", self.tag).unwrap();
        }

        self.renderer
    }
}

pub struct TextRenderer {
    hasher: XxHash32,
    renderer: Renderer,
}

impl TextRenderer {
    pub fn end(mut self) -> Result<Renderer, crate::Error> {
        let hash = self.hasher.finish() as u32;
        self.renderer.hasher.write_u32(hash);

        Ok(self.renderer)
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
