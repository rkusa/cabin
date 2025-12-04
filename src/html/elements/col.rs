use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::void_element::VoidElement;

impl Context {
    /// If a `col` element has a parent and that is a [super::colgroup::Colgroup] element that
    /// itself has a parent that is a [super::table] element, then the `col` element represents
    /// one or more columns in the column group represented by that [super::colgroup::Colgroup].
    pub fn col(&self) -> VoidElement<'_, marker::Col> {
        VoidElement::new(self, "col")
    }
}

pub mod marker {
    pub struct Col;
}

impl<'v> Col for VoidElement<'v, marker::Col> {}
impl<'v> Common for VoidElement<'v, marker::Col> {}
impl<'v> Global for VoidElement<'v, marker::Col> {}
impl<'v> Aria for VoidElement<'v, marker::Col> {}

/// If a `col` element has a parent and that is a [super::colgroup::Colgroup] element that itself
/// has a parent that is a [super::table] element, then the `col` element represents one or more
/// columns in the column group represented by that [super::colgroup::Colgroup].
pub trait Col: WithAttribute {
    /// Number of columns spanned by the element
    fn span(self, width: u32) -> Self {
        self.with_attribute(Span(width))
    }
}

/// Number of columns spanned by the element
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Span(pub u32);
