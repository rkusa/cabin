use std::collections::HashMap;
use std::fmt::{self, Display, Write};
use std::hash::{BuildHasherDefault, Hash, Hasher};

use http::{HeaderMap, HeaderValue};

use crate::View;
use crate::error::InternalError;
use crate::event::Event;
use crate::hash::FxHasher;
use crate::html::events::CustomEvent;
use crate::scope::Scope;
use crate::style::class_name::ClassNames;
use crate::style::{ClassName, StyleDefinition};
use crate::view::RenderFuture;

// This covers about 75% of [Renderer] usages in my largest app as per 2025-12-10.
const DEFAULT_CAPACITY: usize = 128;

pub struct Renderer {
    out: String,
    headers: HeaderMap<HeaderValue>,
    // Keyed by an already hashed class name, so use the fast FxHasher instead of SipHash.
    styles: HashMap<ClassName, Box<StyleDefinition>, BuildHasherDefault<FxHasher>>,
    hasher: FxHasher,
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
            hasher: FxHasher::default(),
            disable_hashes,
            is_update,
        }
    }

    pub fn reset(&mut self) {
        self.out.clear();
        self.headers.clear();
        self.styles.clear();
        self.hasher = FxHasher::default();
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
        let hash = other.hasher.finish_u32();
        self.hasher.write_u32(hash);
        Scope::release_renderer_to_task(other);
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

    pub fn append_style(&mut self, style: Box<StyleDefinition>) -> ClassName {
        let class_name = style.class_name();
        self.styles.entry(class_name).or_insert(style);
        class_name
    }

    pub(crate) fn build_styles(&mut self) -> String {
        let mut styles = self.styles.drain().collect::<Vec<_>>();
        // Sort by class name too, so the output doesn't depend on the map's iteration order.
        styles.sort_by(|(a_name, a), (b_name, b)| {
            a.modifier.cmp(&b.modifier).then(a_name.cmp(b_name))
        });

        let mut css = String::with_capacity(styles.len() * 128);

        for (_, s) in styles {
            s.write_to(&mut css);
        }

        css
    }
}

pub struct ElementRenderer {
    tag: &'static str,
    pub(crate) renderer: Renderer,
    parent_hasher: FxHasher,
    content_started: bool,
    hash_offset: Option<usize>,
}

impl ElementRenderer {
    pub fn attribute(&mut self, name: &str, value: impl Display + Hash) {
        // Can't happen, as `content()` consumes the renderer.
        debug_assert!(!self.content_started, "attribute written after content");
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

    /// Like [Self::attribute], but skips escaping the generated class names, which never need it.
    /// Writing them through the escaping writer was a considerable part of the render time, as
    /// almost every element has a class attribute.
    pub(crate) fn class_names_attribute(&mut self, class_names: &ClassNames) {
        // Can't happen, as `content()` consumes the renderer.
        debug_assert!(!self.content_started, "attribute written after content");
        // Hashed the same way as [Self::attribute] would.
        self.renderer.hasher.write(b"class");
        class_names.hash(&mut self.renderer.hasher);

        self.renderer.out.push_str(r#" class=""#);
        class_names.write_to(&mut self.renderer.out);
        self.renderer.out.push('"');
    }

    pub fn empty_attribute(&mut self, name: &str) {
        // Can't happen, as `content()` consumes the renderer.
        debug_assert!(!self.content_started, "attribute written after content");
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

        let hash = self.renderer.hasher.finish_u32();
        if let Some(offset) = self.hash_offset {
            write!(WriteInto::new(&mut self.renderer.out, offset), "{hash:08x}").unwrap();
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
    hasher: FxHasher,
    renderer: Renderer,
}

impl TextRenderer {
    pub fn end(mut self) -> Result<Renderer, crate::Error> {
        let hash = self.hasher.finish_u32();
        self.renderer.hasher.write_u32(hash);

        Ok(self.renderer)
    }
}

pub struct Escape<W> {
    wr: W,
    mode: EscapeMode,
}

#[derive(Clone, Copy)]
enum EscapeMode {
    AttributeValue,
    Content,
    Script,
}

impl<W> Escape<W> {
    pub fn attribute_value(wr: W) -> Self {
        Escape {
            wr,
            mode: EscapeMode::AttributeValue,
        }
    }

    pub fn content(wr: W) -> Self {
        Escape {
            wr,
            mode: EscapeMode::Content,
        }
    }

    pub fn script(wr: W) -> Self {
        Escape {
            wr,
            mode: EscapeMode::Script,
        }
    }
}

impl<W> fmt::Write for Escape<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Dispatch once per write instead of per byte, so that each loop gets its escape function
        // inlined.
        match self.mode {
            EscapeMode::AttributeValue => write_escaped(&mut self.wr, s, escape_attribute_value),
            EscapeMode::Content => write_escaped(&mut self.wr, s, escape_content),
            EscapeMode::Script => write_escaped(&mut self.wr, s, escape_script),
        }
    }
}

/// Writes `s` to `wr`, replacing each byte for which `escape` returns a replacement. Scans bytes
/// instead of chars, as only ASCII bytes are ever replaced, and those never occur inside a
/// multi-byte UTF-8 sequence. Thus, all slice boundaries are char boundaries.
#[inline(always)]
fn write_escaped(
    wr: &mut impl fmt::Write,
    s: &str,
    escape: impl Fn(u8, &[u8]) -> Option<&'static str>,
) -> fmt::Result {
    let bytes = s.as_bytes();
    let mut pos = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if let Some(sub) = escape(b, &bytes[i + 1..]) {
            if i > pos {
                wr.write_str(&s[pos..i])?;
            }
            wr.write_str(sub)?;
            pos = i + 1;
        }
    }
    if pos == 0 {
        wr.write_str(s)
    } else if pos < s.len() {
        wr.write_str(&s[pos..])
    } else {
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

fn escape_attribute_value(b: u8, _rest: &[u8]) -> Option<&'static str> {
    // Not escaping ' -> because cabin always warps attribute values in double-quotes
    match b {
        b'"' => Some("&quot;"),
        b'&' => Some("&amp;"),
        _ => None,
    }
}

fn escape_content(b: u8, _rest: &[u8]) -> Option<&'static str> {
    match b {
        b'<' => Some("&lt;"),
        b'&' => Some("&amp;"),
        _ => None,
    }
}

/// Only escapes what could end the script element early (`</script`) or switch the tokenizer into
/// its escaped state (`<!--`), in which a later `<script` would hide the closing tag. Anything else
/// is kept as is, so that e.g. `a<!b` or `a</b` in JavaScript stays intact. The script content is
/// written in one go, so a sequence can't be split across two writes.
fn escape_script(b: u8, rest: &[u8]) -> Option<&'static str> {
    if b != b'<' {
        return None;
    }
    let is_script_end =
        rest.len() >= 7 && rest[0] == b'/' && rest[1..7].eq_ignore_ascii_case(b"script");
    if is_script_end || rest.starts_with(b"!--") {
        Some("<\\")
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::h;

    fn escape(escape: fn(String) -> Escape<String>, s: &str) -> String {
        let mut escape = escape(String::new());
        escape.write_str(s).unwrap();
        escape.wr
    }

    #[test]
    fn test_escape() {
        assert_eq!(escape(Escape::content, "äb€"), "äb€");
        assert_eq!(escape(Escape::content, "<ä&>\"€<"), "&lt;ä&amp;>\"€&lt;");
        assert_eq!(
            escape(Escape::attribute_value, "\"ä&<'€\""),
            "&quot;ä&amp;<'€&quot;"
        );
        assert_eq!(
            escape(Escape::script, "ä</script>€<!--"),
            r"ä<\/script>€<\!--"
        );
        assert_eq!(escape(Escape::script, "<"), "<");
    }

    #[tokio::test]
    async fn test_hashes_are_zero_padded() {
        // Without padding, a hash below 0x10000000 would be written left-aligned into the
        // `00000000` placeholder and thus alias another hash, e.g. 0xabcdef1 with 0xabcdef10. So
        // with padding, some of the hashes are expected to start with a zero.
        let mut has_leading_zero = false;
        for i in 0..256 {
            let html = h::div(i.to_string())
                .render(Renderer::new(false, false))
                .await
                .unwrap()
                .end()
                .html;
            let hash = html
                .strip_prefix(r#"<div hash=""#)
                .and_then(|s| s.split_once('"'))
                .unwrap()
                .0;
            assert_eq!(hash.len(), 8, "{html}");
            assert!(hash.bytes().all(|b| b.is_ascii_hexdigit()), "{html}");
            has_leading_zero |= hash.starts_with('0');
        }
        assert!(has_leading_zero);
    }
}
