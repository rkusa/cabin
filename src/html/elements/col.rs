use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::Html;

/// If a `col` element has a parent and that is a [super::Colgroup] element that itself has a parent
/// that is a [super::Table] element, then the `col` element represents one or more columns in the
/// column group represented by that [super::Colgroup].
pub fn col() -> Html<marker::Col, (), ()> {
    Html::new("col", (), ())
}

pub mod marker {
    pub struct Col;
}

impl<A: Attributes, V: 'static> Col for Html<marker::Col, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Col, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Col, A, V> {}

/// If a `col` element has a parent and that is a [super::Colgroup] element that itself has a parent
/// that is a [super::Table] element, then the `col` element represents one or more columns in the
/// column group represented by that [super::Colgroup].
pub trait Col: WithAttribute {
    /// Number of columns spanned by the element
    fn span(self, width: u32) -> Self::Output<Span> {
        self.with_attribute(Span(width))
    }
}

/// Number of columns spanned by the element
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Span(pub u32);
