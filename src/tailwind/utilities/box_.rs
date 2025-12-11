//! Set how the browser should calculate an element's total size (`box-sizing`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>

use crate::tailwind::Property;

const BOX_SIZING: &str = "box-sizing";

/// ```css
/// box-sizing: border-box;
/// ```
pub const BORDER: Property = Property(BOX_SIZING, "border-box");

/// ```css
/// box-sizing: content-box;
/// ```
pub const CONTENT: Property = Property(BOX_SIZING, "content-box");
