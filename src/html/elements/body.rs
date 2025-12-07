use super::common::Common;
use super::global::Global;
use crate::element::Element;

/// The `body` element represents the body of an HTML document.
pub fn body() -> Element<marker::Body> {
    Element::new("body")
}

pub mod marker {
    pub struct Body;
}

impl Common for Element<marker::Body> {}
impl Global for Element<marker::Body> {}
