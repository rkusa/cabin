use super::global::Global;
use crate::element::Element;

/// The `html` element represents the root of an HTML document.
pub fn html() -> Element<marker::Html> {
    Element::new("html")
}

pub mod marker {
    pub struct Html;
}

impl Global for Element<marker::Html> {}
