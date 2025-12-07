use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `html` element represents the root of an HTML document.
    pub fn html(&self) -> Element<marker::Html> {
        Element::new(self.acquire_renderer(), "html")
    }
}

pub mod marker {
    pub struct Html;
}

impl Global for Element<marker::Html> {}
