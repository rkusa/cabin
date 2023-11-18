//! Set how elements are sized and placed across grid columns (`grid-column`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-column>

use std::fmt;

use crate::Property;

const GRID_COLUMN: &str = "grid-column";
const GRID_COLUMN_START: &str = "grid-column-start";
const GRID_COLUMN_END: &str = "grid-column-end";

/// ```css
/// grid-column: auto;
/// ```
pub const AUTO: Property = Property(GRID_COLUMN, "auto");

/// ```css
/// grid-column: 1 / -1;
/// ```
pub const FULL: Property = Property(GRID_COLUMN, "1 / -1");

/// ```css
/// grid-column: span {n} / span {n};
/// ```
pub fn span(n: u16) -> Property<ColSpan> {
    Property(GRID_COLUMN, ColSpan(n))
}

pub struct ColSpan(u16);

impl fmt::Display for ColSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "span {n} / span {n}", n = self.0)
    }
}

/// ```css
/// grid-column-start: {n};
/// ```
pub fn start(n: u16) -> Property<u16> {
    Property(GRID_COLUMN_START, n)
}

pub mod start {
    use crate::Property;

    use super::GRID_COLUMN_START;

    /// ```css
    /// grid-column-start: auto;
    /// ```
    pub const AUTO: Property = Property(GRID_COLUMN_START, "auto");
}

/// ```css
/// grid-column-end: {n};
/// ```
pub fn end(n: u16) -> Property<u16> {
    Property(GRID_COLUMN_END, n)
}

pub mod end {
    use crate::Property;

    use super::GRID_COLUMN_END;

    /// ```css
    /// grid-column-end: auto;
    /// ```
    pub const AUTO: Property = Property(GRID_COLUMN_END, "auto");
}
