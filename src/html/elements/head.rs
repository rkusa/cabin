use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `head` element represents a collection of metadata for the document.
    pub fn head(&self) -> Element<'_, marker::Head> {
        Element::new(self, "head")
    }
}

pub mod marker {
    pub struct Head;
}

impl<'v> Global for Element<'v, marker::Head> {}
