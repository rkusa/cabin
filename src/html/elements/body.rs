use super::common::Common;
use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `body` element represents the body of an HTML document.
    pub fn body(&self) -> Element<marker::Body> {
        Element::new(self.acquire_renderer(), "body")
    }
}

pub mod marker {
    pub struct Body;
}

impl Common for Element<marker::Body> {}
impl Global for Element<marker::Body> {}
