use super::global::Global;
use crate::element::Element;

/// The `head` element represents a collection of metadata for the document.
pub fn head() -> Element<marker::Head> {
    Element::new("head")
}

pub mod marker {
    pub struct Head;
}

impl Global for Element<marker::Head> {}
