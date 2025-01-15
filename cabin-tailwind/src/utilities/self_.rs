//! Define how to justify an invidvidual flex or grid item along its container's cross axis
//! (`align-self`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>

use crate::Property;

const ALIGN_SELF: &str = "align-self";

/// ```css
/// align-self: auto;
/// ```
pub const AUTO: Property = Property(ALIGN_SELF, "auto");

/// ```css
/// align-self: flex-start;
/// ```
pub const START: Property = Property(ALIGN_SELF, "flex-start");

/// ```css
/// align-self: flex-end;
/// ```
pub const END: Property = Property(ALIGN_SELF, "flex-end");

/// ```css
/// align-self: center;
/// ```
pub const CENTER: Property = Property(ALIGN_SELF, "center");

/// ```css
/// align-self: stretch;
/// ```
pub const STRETCH: Property = Property(ALIGN_SELF, "stretch");

/// ```css
/// align-self: baseline;
/// ```
pub const BASELINE: Property = Property(ALIGN_SELF, "baseline");
