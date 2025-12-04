use super::col::Span;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `colgroup` element represents a group of one or more columns in the [super::table] that
    /// is its parent, if it has a parent and that is a [super::table] element.
    pub fn colgroup(&self) -> Element<'_, marker::Colgroup> {
        Element::new(self, "colgroup")
    }
}

pub mod marker {
    pub struct Colgroup;
}

impl<'v> Colgroup for Element<'v, marker::Colgroup> {}
impl<'v> Common for Element<'v, marker::Colgroup> {}
impl<'v> Global for Element<'v, marker::Colgroup> {}

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub trait Colgroup: WithAttribute {
    /// Number of colgroupumns spanned by the element
    fn span(self, width: u32) -> Self {
        self.with_attribute(Span(width))
    }
}
