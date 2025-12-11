//! Set the easing of CSS transitions (`transition-timing-function`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function>

use crate::tailwind::Property;

const TRANSITION_TIMING_FUNCTION: &str = "transition-timing-function";

/// Set the easing function of CSS transitions.
/// ```css
/// transition-timing-function: {func};
/// ```
pub fn custom(func: &'static str) -> Property {
    Property(TRANSITION_TIMING_FUNCTION, func)
}

/// ```css
/// transition-timing-function: linear;
/// ```
pub const LINEAR: Property = Property(TRANSITION_TIMING_FUNCTION, "linear");

/// ```css
/// transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
/// ```
pub const IN: Property = Property(TRANSITION_TIMING_FUNCTION, "cubic-bezier(0.4, 0, 1, 1)");

/// ```css
/// transition-timing-function: cubic-bezier(0, 0, 0.2, 1);
/// ```
pub const OUT: Property = Property(TRANSITION_TIMING_FUNCTION, "cubic-bezier(0, 0, 0.2, 1)");

/// ```css
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// ```
pub const IN_OUT: Property = Property(TRANSITION_TIMING_FUNCTION, "cubic-bezier(0.4, 0, 0.2, 1)");
