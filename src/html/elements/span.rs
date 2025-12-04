use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::context::Context;
use crate::element::Element;

impl Context {
    // TODO:
    pub fn span(&self) -> Element<'_, marker::Span> {
        Element::new(self, "span")
    }
}

pub mod marker {
    pub struct Span;
}

impl<'v> Common for Element<'v, marker::Span> {}
impl<'v> Global for Element<'v, marker::Span> {}
impl<'v> Aria for Element<'v, marker::Span> {}
