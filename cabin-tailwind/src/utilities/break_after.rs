//! Set how page, column, or region breaks should behave after a generated box (`break-after`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/break-after>

use crate::Property;

const BREAK_AFTER: &str = "break-after";

/// ```css
/// break-after: auto;
/// ```
pub const AUTO: Property = Property(BREAK_AFTER, "auto");

/// ```css
/// break-after: avoid;
/// ```
pub const AVOID: Property = Property(BREAK_AFTER, "avoid");

/// ```css
/// break-after: all;
/// ```
pub const ALL: Property = Property(BREAK_AFTER, "all");

/// ```css
/// break-after: avoid-page;
/// ```
pub const AVOID_PAGE: Property = Property(BREAK_AFTER, "avoid-page");

/// ```css
/// break-after: page;
/// ```
pub const PAGE: Property = Property(BREAK_AFTER, "page");

/// ```css
/// break-after: left;
/// ```
pub const LEFT: Property = Property(BREAK_AFTER, "left");

/// ```css
/// break-after: right;
/// ```
pub const RIGHT: Property = Property(BREAK_AFTER, "right");

/// ```css
/// break-after: column;
/// ```
pub const COLUMN: Property = Property(BREAK_AFTER, "column");
