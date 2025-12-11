//! Set how page, column, or region breaks should behave before a generated box (`break-before`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/break-before>

use crate::tailwind::Property;

const BREAK_BEFORE: &str = "break-before";

/// ```css
/// break-before: auto;
/// ```
pub const AUTO: Property = Property(BREAK_BEFORE, "auto");

/// ```css
/// break-before: avoid;
/// ```
pub const AVOID: Property = Property(BREAK_BEFORE, "avoid");

/// ```css
/// break-before: all;
/// ```
pub const ALL: Property = Property(BREAK_BEFORE, "all");

/// ```css
/// break-before: avoid-page;
/// ```
pub const AVOID_PAGE: Property = Property(BREAK_BEFORE, "avoid-page");

/// ```css
/// break-before: page;
/// ```
pub const PAGE: Property = Property(BREAK_BEFORE, "page");

/// ```css
/// break-before: left;
/// ```
pub const LEFT: Property = Property(BREAK_BEFORE, "left");

/// ```css
/// break-before: right;
/// ```
pub const RIGHT: Property = Property(BREAK_BEFORE, "right");

/// ```css
/// break-before: column;
/// ```
pub const COLUMN: Property = Property(BREAK_BEFORE, "column");
