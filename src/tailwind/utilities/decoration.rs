//! Set the decoration of text (`text-decoration-line`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/text-decoration-line>

use crate::tailwind::Property;

const TEXT_DECORATION_LINE: &str = "text-decoration-line";

/// ```css
/// text-decoration-line: underline;
/// ```
pub const UNDERLINE: Property = Property(TEXT_DECORATION_LINE, "underline");

/// ```css
/// text-decoration-line: overline;
/// ```
pub const OVERLINE: Property = Property(TEXT_DECORATION_LINE, "overline");

/// ```css
/// text-decoration-line: line-through;
/// ```
pub const LINE_THROUGH: Property = Property(TEXT_DECORATION_LINE, "line-through");

/// ```css
/// text-decoration-line: none;
/// ```
pub const NO_UNDERLINE: Property = Property(TEXT_DECORATION_LINE, "none");
