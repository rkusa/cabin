//! Set whether whether an element responds to pointer events (`pointer-events`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/pointer-events>

use crate::Property;

const POINTER_EVENTS: &str = "pointer-events";

/// ```css
/// pointer-events: none;
/// ```
pub const NONE: Property = Property(POINTER_EVENTS, "none");

/// ```css
/// pointer-events: auto;
/// ```
pub const AUTO: Property = Property(POINTER_EVENTS, "auto");
