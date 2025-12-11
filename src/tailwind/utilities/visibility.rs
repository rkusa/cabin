//! Set the visibility of an element (`visibility`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/visibility>

use crate::tailwind::Property;

const VISIBILITY: &str = "visibility";

/// ```css
/// visibility: visible;
/// ```
pub const VISIBLE: Property = Property(VISIBILITY, "visible");

/// ```css
/// visibility: hidden;
/// ```
pub const INVISIBLE: Property = Property(VISIBILITY, "hidden");

/// ```css
/// visibility: collapse;
/// ```
pub const COLLAPSE: Property = Property(VISIBILITY, "collapse");
