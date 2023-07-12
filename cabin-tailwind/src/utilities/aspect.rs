//! Set the preferred aspect ratio for the box (`aspect-ratio`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>

use crate::Property;

const ASPECT_RATIO: &str = "aspect-ratio";

/// ```css
/// aspect-ratio: auto;
/// ```
pub const AUTO: Property = Property(ASPECT_RATIO, "auto");

/// ```css
/// aspect-ratio: 1 / 1;
/// ```
pub const SQUARE: Property = Property(ASPECT_RATIO, "1 / 1");

/// ```css
/// aspect-ratio: 19 / 9;
/// ```
pub const VIDEO: Property = Property(ASPECT_RATIO, "16 / 9");

// Custom preferred aspect ratio. Examples: "4 / 3", "0.5".
pub fn ratio(ratio: &'static str) -> Property {
    Property(ASPECT_RATIO, ratio)
}
