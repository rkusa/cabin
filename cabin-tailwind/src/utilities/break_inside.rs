//! Set how page, column, or region breaks should behave within an element (`break-inside`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/break-inside>

use crate::Property;

const BREAK_INSIDE: &str = "break-inside";

/// ```css
/// break-inside: auto;
/// ```
pub const AUTO: Property = Property(BREAK_INSIDE, "auto");

/// ```css
/// break-inside: avoid;
/// ```
pub const AVOID: Property = Property(BREAK_INSIDE, "avoid");

/// ```css
/// break-inside: page;
/// ```
pub const PAGE: Property = Property(BREAK_INSIDE, "page");

/// ```css
/// break-inside: column;
/// ```
pub const COLUMN: Property = Property(BREAK_INSIDE, "column");
