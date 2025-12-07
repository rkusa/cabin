use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `head` element represents a collection of metadata for the document.
    pub fn head(&self) -> Element<marker::Head> {
        Element::new(self.acquire_renderer(), "head")
    }
}

pub mod marker {
    pub struct Head;
}

impl Global for Element<marker::Head> {}
