use super::col::Span;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `colgroup` element represents a group of one or more columns in the [super::table] that
/// is its parent, if it has a parent and that is a [super::table] element.
pub fn colgroup() -> Element<marker::Colgroup> {
    Element::new("colgroup")
}

pub mod marker {
    pub struct Colgroup;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Colgroup> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Colgroup<(marker::Colgroup, P)> for E where E: ElementProxy<marker::Colgroup, P> {}
impl<E, P> Common<(marker::Colgroup, P)> for E where E: ElementProxy<marker::Colgroup, P> {}
impl<E, P> Global<(marker::Colgroup, P)> for E where E: ElementProxy<marker::Colgroup, P> {}

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub trait Colgroup<T>: WithAttribute {
    /// Number of colgroupumns spanned by the element
    fn span(self, width: u32) -> Self {
        self.with_attribute(Span(width))
    }
}
