//! Set how white space inside an element is handled (`white-space`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/white-space>

use crate::tailwind::Property;

const WHITESPACE: &str = "white-space";

/// ```css
/// white-space: normal;
/// ```
pub const NORMAL: Property = Property(WHITESPACE, "normal");

/// ```css
/// white-space: nowrap;
/// ```
pub const NOWRAP: Property = Property(WHITESPACE, "nowrap");

/// ```css
/// white-space: pre;
/// ```
pub const PRE: Property = Property(WHITESPACE, "pre");

/// ```css
/// white-space: pre-line;
/// ```
pub const PRE_LINE: Property = Property(WHITESPACE, "pre-line");

/// ```css
/// white-space: pre-wrap;
/// ```
pub const PRE_WRAP: Property = Property(WHITESPACE, "pre-wrap");

/// ```css
/// white-space: break-spaces;
/// ```
pub const BREAK_SPACES: Property = Property(WHITESPACE, "break-spaces");
