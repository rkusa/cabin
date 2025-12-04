use super::common::Common;
use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `body` element represents the body of an HTML document.
    pub fn body(&self) -> Element<'_, marker::Body> {
        Element::new(self, "body")
    }
}

pub mod marker {
    pub struct Body;
}

impl<'v> Common for Element<'v, marker::Body> {}
impl<'v> Global for Element<'v, marker::Body> {}
