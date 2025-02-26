use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use super::td::{ColSpan, Headers, RowSpan};
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::list::SpaceSeparated;
use crate::html::{Aria, Html};

/// The `th` element represents a header cell in a [super::table].
pub fn th(content: impl View) -> Html<marker::Th, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("th", (), content)
}

pub mod marker {
    pub struct Th;
}

impl<A: Attributes, V: 'static> Th for Html<marker::Th, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Th, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Th, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Th, A, V> {}

/// The `th` element represents a header cell in a [super::table].
pub trait Th: WithAttribute {
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

    /// Specifies which cells the header cell applies to.
    fn scope(self, scope: Scope) -> Self::Output<Scope> {
        self.with_attribute(scope)
    }

    /// Alternative label to use for the header cell when referencing the cell in other contexts.
    fn abbr(self, abbr: impl Into<Cow<'static, str>>) -> Self::Output<Abbr> {
        self.with_attribute(Abbr(abbr.into()))
    }
}

/// Specifies which cells the header cell applies to.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Scope {
    /// The header cell applies to some of the subsequent cells in the same row(s).
    Row,

    /// The header cell applies to some of the subsequent cells in the same column(s).
    Col,

    /// The header cell applies to all the remaining cells in the row group. A [th] element's
    /// [Th::scope] attribute must not be in the row group state if the element is not anchored in
    /// a row group.
    RowGroup,

    /// The header cell applies to all the remaining cells in the column group. A [th] element's
    /// [Th::scope] attribute must not be in the column group state if the element is not anchored
    /// in a column group.
    ColGroup,

    /// The header cell apply to a set of cells selected based on context.
    #[default]
    Auto,
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Row => "row",
            Self::Col => "col",
            Self::RowGroup => "rowgroup",
            Self::ColGroup => "colgroup",
            Self::Auto => "auto",
        })
    }
}

/// Alternative label to use for the header cell when referencing the cell in other contexts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Abbr(pub Cow<'static, str>);
