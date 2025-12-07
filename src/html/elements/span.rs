use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    // TODO:
    pub fn span(&self) -> Element<marker::Span> {
        Element::new(self.acquire_renderer(), "span")
    }
}

pub mod marker {
    pub struct Span;
}

impl Common for Element<marker::Span> {}
impl Global for Element<marker::Span> {}
impl Aria for Element<marker::Span> {}
