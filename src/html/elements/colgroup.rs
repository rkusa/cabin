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

impl<P> Colgroup<marker::Colgroup> for P where P: ElementProxy<marker::Colgroup> {}
impl<P> Common<marker::Colgroup> for P where P: ElementProxy<marker::Colgroup> {}
impl<P> Global<marker::Colgroup> for P where P: ElementProxy<marker::Colgroup> {}

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub trait Colgroup<T>: WithAttribute {
    /// Number of colgroupumns spanned by the element
    fn span(self, width: u32) -> Self {
        self.with_attribute(Span(width))
    }
}
