use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::list::SpaceSeparated;
use crate::html::{Aria, Html};

/// The `td` element represents a data cell in a [super::table].
#[crate::view_macro(cabin::html::elements::td)]
pub fn td(content: impl View) -> Html<marker::Td, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("td", (), content)
}

pub mod marker {
    pub struct Td;
}

impl<A: Attributes> Td for Html<marker::Td, A> {}
impl<A: Attributes> Common for Html<marker::Td, A> {}
impl<A: Attributes> Global for Html<marker::Td, A> {}
impl<A: Attributes> Aria for Html<marker::Td, A> {}

/// The `td` element represents a data cell in a [super::table].
pub trait Td: WithAttribute {
    /// Number of columns that the cell is to span.
    fn col_span(self, col_span: u32) -> Self::Output<ColSpan> {
        self.with_attribute(ColSpan(col_span))
    }

    /// Number of rows that the cell is to span.
    fn row_span(self, row_span: u32) -> Self::Output<RowSpan> {
        self.with_attribute(RowSpan(row_span))
    }

    /// The header cells for this cell.
    fn headers(
        self,
        headers: impl Into<SpaceSeparated<Cow<'static, str>>>,
    ) -> Self::Output<Headers> {
        self.with_attribute(Headers(headers.into()))
    }

    /// Appends a header cell for this cell.
    fn append_header(mut self, header: impl Into<Cow<'static, str>>) -> Self::Output<Headers> {
        let headers = if let Some(list) = self.get_attribute_mut::<Headers>() {
            Headers(
                match std::mem::replace(&mut list.0, SpaceSeparated::Single(Cow::Borrowed(""))) {
                    SpaceSeparated::Single(existing) => {
                        SpaceSeparated::List([existing, header.into()].into())
                    }
                    SpaceSeparated::List(mut list) => {
                        list.insert(header.into());
                        SpaceSeparated::List(list)
                    }
                },
            )
        } else {
            Headers(SpaceSeparated::Single(header.into()))
        };
        self.with_attribute(headers)
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
