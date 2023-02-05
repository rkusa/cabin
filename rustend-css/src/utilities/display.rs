//! Set whether an element is treated as a block or inline element and the layout used for its
//! children, such as flow layout, grid or flex (`display`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/display>

use crate::Property;

const DISPLAY: &str = "display";

/// `display: block;`
pub const BLOCK: Property = Property(DISPLAY, "block");

/// `display: inline-block;`
pub const INLINE_BLOCK: Property = Property(DISPLAY, "inline-block");

/// `display: inline;`
pub const INLINE: Property = Property(DISPLAY, "inline");

/// `display: flex;`
pub const FLEX: Property = Property(DISPLAY, "flex");

/// `display: inline-flex;`
pub const INLINE_FLEX: Property = Property(DISPLAY, "inline-flex");

/// `display: table;`
pub const TABLE: Property = Property(DISPLAY, "table");

/// `display: inline-table;`
pub const INLINE_TABLE: Property = Property(DISPLAY, "inline-table");

/// `display: table-caption;`
pub const TABLE_CAPTION: Property = Property(DISPLAY, "table-caption");

/// `display: table-cell;`
pub const TABLE_CELL: Property = Property(DISPLAY, "table-cell");

/// `display: table-column;`
pub const TABLE_COLUMN: Property = Property(DISPLAY, "table-column");

/// `display: table-column-group;`
pub const TABLE_COLUMN_GROUP: Property = Property(DISPLAY, "table-column-group");

/// `display: table-footer-group;`
pub const TABLE_FOOTER_GROUP: Property = Property(DISPLAY, "table-footer-group");

/// `display: table-header-group;`
pub const TABLE_HEADER_GROUP: Property = Property(DISPLAY, "table-header-group");

/// `display: table-row-group;`
pub const TABLE_ROW_GROUP: Property = Property(DISPLAY, "table-row-group");

/// `display: table-row;`
pub const TABLE_ROW: Property = Property(DISPLAY, "table-row");

/// `display: flow-root;`
pub const FLOW_ROOT: Property = Property(DISPLAY, "flow-root");

/// `display: grid;`
pub const GRID: Property = Property(DISPLAY, "grid");

/// `display: inline-grid;`
pub const INLINE_GRID: Property = Property(DISPLAY, "inline-grid");

/// `display: contents;`
pub const CONTENTS: Property = Property(DISPLAY, "contents");

/// `display: list-item;`
pub const LIST_ITEM: Property = Property(DISPLAY, "list-item");

/// `display: none;`
pub const HIDDEN: Property = Property(DISPLAY, "none");
