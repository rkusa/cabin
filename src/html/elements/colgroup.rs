use super::col::Span;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// The `colgroup` element represents a group of one or more columns in the [super::table] that
/// is its parent, if it has a parent and that is a [super::table] element.
pub fn colgroup() -> Element<marker::Colgroup> {
    Element::new("colgroup")
}

pub mod marker {
    pub struct Colgroup;
}

impl Colgroup for Element<marker::Colgroup> {}
impl Common for Element<marker::Colgroup> {}
impl Global for Element<marker::Colgroup> {}

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub trait Colgroup: WithAttribute {
    /// Number of colgroupumns spanned by the element
    fn span(self, width: u32) -> Self {
        self.with_attribute(Span(width))
    }
}
