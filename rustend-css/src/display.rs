//! Utilities for controlling the display box type of an element.

use internal::Display;

/// Displays the element as `block` (css `display` property).
pub const BLOCK: Display = Display("block");

/// Displays the element as `inline-block` (css `display` property).
pub const INLINE_BLOCK: Display = Display("inline-block");

/// Displays the element as `inline` (css `display` property).
pub const INLINE: Display = Display("inline");

/// Displays the element as `flex` (css `display` property).
pub const FLEX: Display = Display("flex");

/// Displays the element as `inline-flex` (css `display` property).
pub const INLINE_FLEX: Display = Display("inline-flex");

/// Displays the element as `table` (css `display` property).
pub const TABLE: Display = Display("table");

/// Displays the element as `inline-table` (css `display` property).
pub const INLINE_TABLE: Display = Display("inline-table");

/// Displays the element as `table-caption` (css `display` property).
pub const TABLE_CAPTION: Display = Display("table-caption");

/// Displays the element as `table-cell` (css `display` property).
pub const TABLE_CELL: Display = Display("table-cell");

/// Displays the element as `table-column` (css `display` property).
pub const TABLE_COLUMN: Display = Display("table-column");

/// Displays the element as `table-column-group` (css `display` property).
pub const TABLE_COLUMN_GROUP: Display = Display("table-column-group");

/// Displays the element as `table-footer-group` (css `display` property).
pub const TABLE_FOOTER_GROUP: Display = Display("table-footer-group");

/// Displays the element as `table-header-group` (css `display` property).
pub const TABLE_HEADER_GROUP: Display = Display("table-header-group");

/// Displays the element as `table-row-group` (css `display` property).
pub const TABLE_ROW_GROUP: Display = Display("table-row-group");

/// Displays the element as `table-row` (css `display` property).
pub const TABLE_ROW: Display = Display("table-row");

/// Displays the element as `flow-root` (css `display` property).
pub const FLOW_ROOT: Display = Display("flow-root");

/// Displays the element as `grid` (css `display` property).
pub const GRID: Display = Display("grid");

/// Displays the element as `inline-grid` (css `display` property).
pub const INLINE_GRID: Display = Display("inline-grid");

/// Displays the element as `contents` (css `display` property).
pub const CONTENTS: Display = Display("contents");

/// Displays the element as `list-item` (css `display` property).
pub const LIST_ITEM: Display = Display("list-item");

/// Displays the element as `none` (css `display` property).
pub const HIDDEN: Display = Display("none");

mod internal {
    use std::fmt;

    use crate::Style;

    pub struct Display(pub(crate) &'static str);

    impl Style for Display {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "display: {};", self.0)
        }
    }
}
