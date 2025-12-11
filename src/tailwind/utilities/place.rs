//! Set how an individual item is justified and aligned at the same time (`place-self`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/place-self>

pub mod content;
pub mod items;

use crate::tailwind::Property;

const PLACE_SELF: &str = "place-self";

/// ```css
/// place-self: auto;
/// ```
pub const AUTO: Property = Property(PLACE_SELF, "auto");

/// ```css
/// place-self: start;
/// ```
pub const START: Property = Property(PLACE_SELF, "start");

/// ```css
/// place-self: end;
/// ```
pub const END: Property = Property(PLACE_SELF, "end");

/// ```css
/// place-self: center;
/// ```
pub const CENTER: Property = Property(PLACE_SELF, "center");

/// ```css
/// place-self: stretch;
/// ```
pub const STRETCH: Property = Property(PLACE_SELF, "stretch");
