use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::void_element::{VoidElement, VoidElementProxy};

/// If a `col` element has a parent and that is a [super::colgroup::Colgroup] element that
/// itself has a parent that is a [super::table] element, then the `col` element represents
/// one or more columns in the column group represented by that [super::colgroup::Colgroup].
pub fn col() -> VoidElement<marker::Col> {
    VoidElement::new("col")
}

pub mod marker {
    pub struct Col;
}

impl<E> Col<marker::Col> for E where E: VoidElementProxy<marker::Col> {}
impl<E> Common<marker::Col> for E where E: VoidElementProxy<marker::Col> {}
impl<E> Global<marker::Col> for E where E: VoidElementProxy<marker::Col> {}
impl<E> Aria<marker::Col> for E where E: VoidElementProxy<marker::Col> {}

/// If a `col` element has a parent and that is a [super::colgroup::Colgroup] element that itself
/// has a parent that is a [super::table] element, then the `col` element represents one or more
/// columns in the column group represented by that [super::colgroup::Colgroup].
pub trait Col<T>: WithAttribute {
    /// Number of columns spanned by the element
    fn span(self, width: u32) -> Self {
        self.with_attribute(Span(width))
    }
}

/// Number of columns spanned by the element
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Span(pub u32);
