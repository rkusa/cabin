//! Set how items are aligned along both block and inline directions (`place-items`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/place-items>

use crate::Property;

const PLACE_ITEMS: &str = "place-items";

/// ```css
/// place-items: start;
/// ```
pub const START: Property = Property(PLACE_ITEMS, "start");

/// ```css
/// place-items: end;
/// ```
pub const END: Property = Property(PLACE_ITEMS, "end");

/// ```css
/// place-items: center;
/// ```
pub const CENTER: Property = Property(PLACE_ITEMS, "center");

/// ```css
/// place-items: baseline;
/// ```
pub const BASELINE: Property = Property(PLACE_ITEMS, "baseline");

/// ```css
/// place-items: stretch;
/// ```
pub const STRETCH: Property = Property(PLACE_ITEMS, "stretch");
