//! Define how to justify a box inside its alignment container along the appropriate axis
//! (`justify-self`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>

use crate::Property;

const JUSTIFY_SELF: &str = "justify-self";

/// ```css
/// justify-self: auto;
/// ```
pub const AUTO: Property = Property(JUSTIFY_SELF, "auto");

/// ```css
/// justify-self: start;
/// ```
pub const START: Property = Property(JUSTIFY_SELF, "start");

/// ```css
/// justify-self: end;
/// ```
pub const END: Property = Property(JUSTIFY_SELF, "end");

/// ```css
/// justify-self: center;
/// ```
pub const CENTER: Property = Property(JUSTIFY_SELF, "center");

/// ```css
/// justify-self: stretch;
/// ```
pub const STRETCH: Property = Property(JUSTIFY_SELF, "stretch");
