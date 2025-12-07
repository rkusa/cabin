use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::element::Element;

// TODO:
pub fn span() -> Element<marker::Span> {
    Element::new("span")
}

pub mod marker {
    pub struct Span;
}

impl Common for Element<marker::Span> {}
impl Global for Element<marker::Span> {}
impl Aria for Element<marker::Span> {}
