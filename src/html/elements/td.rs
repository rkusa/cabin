use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};
use crate::html::list::SpaceSeparated;

/// The `td` element represents a data cell in a [super::table].
pub fn td() -> Element<marker::Td> {
    Element::new("td")
}

pub mod marker {
    pub struct Td;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Td> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Td<(marker::Td, P)> for E where E: ElementProxy<marker::Td, P> {}
impl<E, P> Common<(marker::Td, P)> for E where E: ElementProxy<marker::Td, P> {}
impl<E, P> Global<(marker::Td, P)> for E where E: ElementProxy<marker::Td, P> {}
impl<E, P> Aria<(marker::Td, P)> for E where E: ElementProxy<marker::Td, P> {}

/// The `td` element represents a data cell in a [super::table].
pub trait Td<T>: WithAttribute {
    /// Number of columns that the cell is to span.
    fn col_span(self, col_span: u32) -> Self {
        self.with_attribute(ColSpan(col_span))
    }

    /// Number of rows that the cell is to span.
    fn row_span(self, row_span: u32) -> Self {
        self.with_attribute(RowSpan(row_span))
    }

    /// The header cells for this cell.
    fn headers(self, headers: impl Into<SpaceSeparated<Cow<'static, str>>>) -> Self {
        self.with_attribute(Headers(headers.into()))
    }
}

/// Number of columns that the cell is to span.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ColSpan(pub u32);

/// Number of rows that the cell is to span.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct RowSpan(pub u32);

/// The header cells for a cell.
#[derive(Debug, Clone, Hash, Attribute)]
pub struct Headers(pub SpaceSeparated<Cow<'static, str>>);
