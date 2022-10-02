mod marker;
#[cfg(test)]
mod test;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;
use std::ops::Neg;

use twox_hash::XxHash32;

use self::marker::Marker;
pub use self::marker::ViewHashTree;
use crate::View;

pub struct Renderer {
    out: String,
    hash_tree: Vec<Marker>,
    hasher: XxHash32,
    previous_tree: Option<Vec<Marker>>,
    previous_offset: isize,
}

pub(crate) struct Out {
    pub view: String,
    pub hash_tree: ViewHashTree,
}

impl Renderer {
    pub(crate) fn new() -> Self {
        Renderer {
            out: String::with_capacity(256),
            hash_tree: Vec::with_capacity(32),
            hasher: XxHash32::default(),
            previous_tree: None,
            previous_offset: 0,
        }
    }

    pub(crate) fn from_previous_tree(previous_tree: ViewHashTree) -> Self {
        Self {
            previous_tree: Some(previous_tree.0),
            ..Self::new()
        }
    }

    pub(crate) fn end(mut self) -> Out {
        self.hash_tree.push(Marker::End(self.finish() as u32));
        Out {
            view: self.out,
            hash_tree: ViewHashTree(self.hash_tree),
        }
    }

    pub fn is_update(&self) -> bool {
        self.previous_tree.is_some()
    }

    pub fn element(mut self, tag: &'static str) -> Result<ElementRenderer, fmt::Error> {
        self.start();

        let parent_hasher = std::mem::take(&mut self.hasher);
        self.write(tag.as_bytes());
        let offset = self.out.len();
        write!(&mut self.out, "<{}", tag)?;
        Ok(ElementRenderer {
            tag,
            offset,
            parent_hasher,
            renderer: self,
            content_started: false,
        })
    }

    pub fn text(&mut self, text: &str) -> Result<(), fmt::Error> {
        self.start();

        let mut hasher = XxHash32::default();
        hasher.write(text.as_bytes());
        let hash = hasher.finish() as u32;
        self.write_u32(hash);
        self.hash_tree.push(Marker::End(hash));

        if !self.unchanged(hash, self.out.len())? {
            self.out.write_str(text)?;
        }

        Ok(())
    }

    fn start(&mut self) {
        let previous = self
            .previous_tree
            .as_ref()
            .and_then(|t| t.get(self.next_position()));
        if let Some(Marker::End(_)) = previous {
            self.previous_offset += 2
        }

        self.hash_tree.push(Marker::Start);
    }

    fn unchanged(&mut self, hash: u32, offset: usize) -> Result<bool, fmt::Error> {
        let previous_position = self.next_position() - 1;
        let mut previous = self
            .previous_tree
            .as_mut()
            .and_then(|t| t.get_mut(previous_position));
        match previous.as_mut() {
            // Subtree did not change
            Some(Marker::End(previous)) => {
                let unchanged = *previous == hash;

                // When the new tree has new items, it is compared to previous values in the old
                // tree (due to the offset). To ensure that they never match, set the old tree
                // hashes to 0 here.
                *previous = 0;

                if unchanged {
                    // TODO: any way to not write the content until the changed detection happens?
                    self.out.truncate(offset);
                    self.out.write_str("<!--unchanged-->")?;

                    return Ok(true);
                }
            }
            // Encountered start marker, which means that the new tree has new items. Update the
            // offset accordingly.
            Some(Marker::Start) => self.previous_offset -= 2,
            _ => {}
        }

        Ok(false)
    }

    fn next_position(&self) -> usize {
        if self.previous_offset > 0 {
            self.hash_tree.len() - self.previous_offset as usize
        } else {
            self.hash_tree.len() + self.previous_offset.neg() as usize
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

    pub async fn content<M>(mut self, view: impl View<M>) -> Result<Renderer, fmt::Error> {
        if is_void_element(self.tag) {
            todo!("throw error: void tags cannot have content");
        }
        if !self.content_started {
            self.content_started = true;
            write!(&mut self.renderer.out, ">")?;
        }

        self.renderer = view.render(self.renderer).await?;
        self.end()
    }

    pub fn end(mut self) -> Result<Renderer, fmt::Error> {
        if !self.content_started && !is_void_element(self.tag) {
            write!(&mut self.renderer.out, ">")?;
        }

        let hash = self.renderer.finish() as u32;
        self.renderer.hash_tree.push(Marker::End(hash));
        self.parent_hasher.write_u32(hash);
        std::mem::swap(&mut self.renderer.hasher, &mut self.parent_hasher);

        if !self.renderer.unchanged(hash, self.offset)? {
            // Handle void elements. Content is simply ignored.
            if is_void_element(self.tag) {
                write!(&mut self.renderer.out, "/>")?;
            } else {
                write!(&mut self.renderer.out, "</{}>", self.tag)?;
            }
        }

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

impl Write for Renderer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.text(s)
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
