//! Utilities for controlling the display box type of an element.

use internal::Display;

/// ```
/// display: block;
/// ```
pub const BLOCK: Display = Display("block");

/// ```
/// display: inline-block;
/// ```
pub const INLINE_BLOCK: Display = Display("inline-block");

/// ```
/// display: inline;
/// ```
pub const INLINE: Display = Display("inline");

/// ```
/// display: flex;
/// ```
pub const FLEX: Display = Display("flex");

/// ```
/// display: inline-flex;
/// ```
pub const INLINE_FLEX: Display = Display("inline-flex");

/// ```
/// display: table;
/// ```
pub const TABLE: Display = Display("table");

/// ```
/// display: inline-table;
/// ```
pub const INLINE_TABLE: Display = Display("inline-table");

/// ```
/// display: table-caption;
/// ```
pub const TABLE_CAPTION: Display = Display("table-caption");

/// ```
/// display: table-cell;
/// ```
pub const TABLE_CELL: Display = Display("table-cell");

/// ```
/// display: table-column;
/// ```
pub const TABLE_COLUMN: Display = Display("table-column");

/// ```
/// display: table-column-group;
/// ```
pub const TABLE_COLUMN_GROUP: Display = Display("table-column-group");

/// ```
/// display: table-footer-group;
/// ```
pub const TABLE_FOOTER_GROUP: Display = Display("table-footer-group");

/// ```
/// display: table-header-group;
/// ```
pub const TABLE_HEADER_GROUP: Display = Display("table-header-group");

/// ```
/// display: table-row-group;
/// ```
pub const TABLE_ROW_GROUP: Display = Display("table-row-group");

/// ```
/// display: table-row;
/// ```
pub const TABLE_ROW: Display = Display("table-row");

/// ```
/// display: flow-root;
/// ```
pub const FLOW_ROOT: Display = Display("flow-root");

/// ```
/// display: grid;
/// ```
pub const GRID: Display = Display("grid");

/// ```
/// display: inline-grid;
/// ```
pub const INLINE_GRID: Display = Display("inline-grid");

/// ```
/// display: contents;
/// ```
pub const CONTENTS: Display = Display("contents");

/// ```
/// display: list-item;
/// ```
pub const LIST_ITEM: Display = Display("list-item");

/// ```
/// display: none;
/// ```
pub const HIDDEN: Display = Display("none");

mod internal {
    use std::fmt;

    use crate::Style;

    /// Element's _display type_ ([`display`]).
    /// [`display`]: https://w3c.github.io/csswg-drafts/css-display/#the-display-properties
    pub struct Display(pub(super) &'static str);

    impl Style for Display {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "display: {};", self.0)
        }
    }
}
