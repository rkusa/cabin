//! Set how content is aligned along both block and inline directions (`place-content`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/place-content>

use crate::Property;

const PLACE_CONTENT: &str = "place-content";

/// ```css
/// place-content: center;
/// ```
pub const CENTER: Property = Property(PLACE_CONTENT, "center");

/// ```css
/// place-content: start;
/// ```
pub const START: Property = Property(PLACE_CONTENT, "start");

/// ```css
/// place-content: end;
/// ```
pub const END: Property = Property(PLACE_CONTENT, "end");

/// ```css
/// place-content: space-between;
/// ```
pub const BETWEEN: Property = Property(PLACE_CONTENT, "space-between");

/// ```css
/// place-content: space-around;
/// ```
pub const AROUND: Property = Property(PLACE_CONTENT, "space-around");

/// ```css
/// place-content: space-evenly;
/// ```
pub const EVENLY: Property = Property(PLACE_CONTENT, "space-evenly");

/// ```css
/// place-content: baseline;
/// ```
pub const BASELINE: Property = Property(PLACE_CONTENT, "baseline");

/// ```css
/// place-content: stretch;
/// ```
pub const STRETCH: Property = Property(PLACE_CONTENT, "stretch");
