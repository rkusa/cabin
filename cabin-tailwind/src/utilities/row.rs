//! Set how elements are sized and placed across grid rows (`grid-row`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-row>

use std::fmt;

use crate::Property;

const GRID_ROW: &str = "grid-row";
const GRID_ROW_START: &str = "grid-row-start";
const GRID_ROW_END: &str = "grid-row-end";

/// ```css
/// grid-row: auto;
/// ```
pub const AUTO: Property = Property(GRID_ROW, "auto");

/// ```css
/// grid-row: 1 / -1;
/// ```
pub const FULL: Property = Property(GRID_ROW, "1 / -1");

/// ```css
/// grid-row: {n};
/// ```
pub fn row(n: i16) -> Property<i16> {
    Property(GRID_ROW, n)
}

/// ```css
/// grid-row: span {n} / span {n};
/// ```
pub fn span(n: u16) -> Property<ColSpan> {
    Property(GRID_ROW, ColSpan(n))
}

pub struct ColSpan(u16);

impl fmt::Display for ColSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "span {n} / span {n}", n = self.0)
    }
}

/// ```css
/// grid-row-start: {n};
/// ```
pub fn start(n: u16) -> Property<u16> {
    Property(GRID_ROW_START, n)
}

pub mod start {
    use super::GRID_ROW_START;
    use crate::Property;

    /// ```css
    /// grid-row-start: auto;
    /// ```
    pub const AUTO: Property = Property(GRID_ROW_START, "auto");
}

/// ```css
/// grid-row-end: {n};
/// ```
pub fn end(n: u16) -> Property<u16> {
    Property(GRID_ROW_END, n)
}

pub mod end {
    use super::GRID_ROW_END;
    use crate::Property;

    /// ```css
    /// grid-row-end: auto;
    /// ```
    pub const AUTO: Property = Property(GRID_ROW_END, "auto");
}
