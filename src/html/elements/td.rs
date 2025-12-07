use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;
use crate::html::list::SpaceSeparated;

/// The `td` element represents a data cell in a [super::table].
pub fn td() -> Element<marker::Td> {
    Element::new("td")
}

pub mod marker {
    pub struct Td;
}

impl Td for Element<marker::Td> {}
impl Common for Element<marker::Td> {}
impl Global for Element<marker::Td> {}
impl Aria for Element<marker::Td> {}

/// The `td` element represents a data cell in a [super::table].
pub trait Td: WithAttribute {
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
