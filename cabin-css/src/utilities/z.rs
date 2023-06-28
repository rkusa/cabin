//! Set the z-order of a positioned element and its descendants or flex items (`z-index`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/z-index>

use crate::{Length, Property};

const Z_INDEX: &str = "z-index";

/// ```css
/// z-index: auto;
/// ```
pub const AUTO: Property<Length> = Property(Z_INDEX, Length::Auto);

/// ```css
/// z-index: {x};
/// ```
pub fn index(x: i16) -> Property<i16> {
    Property(Z_INDEX, x)
}
