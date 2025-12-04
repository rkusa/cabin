use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `html` element represents the root of an HTML document.
    pub fn html(&self) -> Element<'_, marker::Html> {
        Element::new(self, "html")
    }
}

pub mod marker {
    pub struct Html;
}

impl<'v> Global for Element<'v, marker::Html> {}
