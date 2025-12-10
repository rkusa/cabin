use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// If a `col` element has a parent and that is a [super::colgroup::Colgroup] element that itself
/// has a parent that is a [super::table] element, then the `col` element represents one or more
/// columns in the column group represented by that [super::colgroup::Colgroup].
pub fn col() -> Html<marker::Col, ()> {
    Html::new("col", (), ()).into_void_element()
}

pub mod marker {
    pub struct Col;
}

impl<A: Attributes> Col for Html<marker::Col, A> {}
impl<A: Attributes> Common for Html<marker::Col, A> {}
impl<A: Attributes> Global for Html<marker::Col, A> {}
impl<A: Attributes> Aria for Html<marker::Col, A> {}

/// If a `col` element has a parent and that is a [super::colgroup::Colgroup] element that itself
/// has a parent that is a [super::table] element, then the `col` element represents one or more
/// columns in the column group represented by that [super::colgroup::Colgroup].
pub trait Col: WithAttribute {
    /// Number of columns spanned by the element
    fn span(self, width: u32) -> Self::Output<Span> {
        self.with_attribute(Span(width))
    }
}

/// Number of columns spanned by the element
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Span(pub u32);
