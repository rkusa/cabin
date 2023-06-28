//! Set whether an element is treated as a block or inline element and the layout used for its
//! children, such as flow layout, grid or flex (`display`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/display>

use crate::Property;

const DISPLAY: &str = "display";

/// ```css
/// display: block;
/// ```
pub const BLOCK: Property = Property(DISPLAY, "block");

/// ```css
/// display: inline-block;
/// ```
pub const INLINE_BLOCK: Property = Property(DISPLAY, "inline-block");

/// ```css
/// display: inline;
/// ```
pub const INLINE: Property = Property(DISPLAY, "inline");

/// ```css
/// display: flex;
/// ```
pub const FLEX: Property = Property(DISPLAY, "flex");

/// ```css
/// display: inline-flex;
/// ```
pub const INLINE_FLEX: Property = Property(DISPLAY, "inline-flex");

/// ```css
/// display: table;
/// ```
pub const TABLE: Property = Property(DISPLAY, "table");

/// ```css
/// display: inline-table;
/// ```
pub const INLINE_TABLE: Property = Property(DISPLAY, "inline-table");

/// ```css
/// display: table-caption;
/// ```
pub const TABLE_CAPTION: Property = Property(DISPLAY, "table-caption");

/// ```css
/// display: table-cell;
/// ```
pub const TABLE_CELL: Property = Property(DISPLAY, "table-cell");

/// ```css
/// display: table-column;
/// ```
pub const TABLE_COLUMN: Property = Property(DISPLAY, "table-column");

/// ```css
/// display: table-column-group;
/// ```
pub const TABLE_COLUMN_GROUP: Property = Property(DISPLAY, "table-column-group");

/// ```css
/// display: table-footer-group;
/// ```
pub const TABLE_FOOTER_GROUP: Property = Property(DISPLAY, "table-footer-group");

/// ```css
/// display: table-header-group;
/// ```
pub const TABLE_HEADER_GROUP: Property = Property(DISPLAY, "table-header-group");

/// ```css
/// display: table-row-group;
/// ```
pub const TABLE_ROW_GROUP: Property = Property(DISPLAY, "table-row-group");

/// ```css
/// display: table-row;
/// ```
pub const TABLE_ROW: Property = Property(DISPLAY, "table-row");

/// ```css
/// display: flow-root;
/// ```
pub const FLOW_ROOT: Property = Property(DISPLAY, "flow-root");

/// ```css
/// display: grid;
/// ```
pub const GRID: Property = Property(DISPLAY, "grid");

/// ```css
/// display: inline-grid;
/// ```
pub const INLINE_GRID: Property = Property(DISPLAY, "inline-grid");

/// ```css
/// display: contents;
/// ```
pub const CONTENTS: Property = Property(DISPLAY, "contents");

/// ```css
/// display: list-item;
/// ```
pub const LIST_ITEM: Property = Property(DISPLAY, "list-item");

/// ```css
/// display: none;
/// ```
pub const HIDDEN: Property = Property(DISPLAY, "none");
