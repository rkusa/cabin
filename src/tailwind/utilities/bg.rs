//! Set the background color an element (`background-color`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/background-color>

pub mod clip;

use crate::tailwind::Property;

const BACKGROUND_COLOR: &str = "background-color";
const BACKGROUND_IMAGE: &str = "background-image";

include!(concat!(env!("OUT_DIR"), "/background-color.rs"));

/// Set a custom background color.
pub fn color(color: &'static str) -> Property {
    Property(BACKGROUND_COLOR, color)
}

/// ```css
/// background-image: none;
/// ```
pub const NONE: Property = Property(BACKGROUND_IMAGE, "none");

/// ```css
/// background-image: linear-gradient(to top, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_T: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to top, var(--tw-gradient-stops))",
);

/// ```css
/// background-image: linear-gradient(to top right, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_TR: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to top right, var(--tw-gradient-stops))",
);

/// ```css
/// background-image: linear-gradient(to right, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_R: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to right, var(--tw-gradient-stops))",
);

/// ```css
/// background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_BR: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to bottom right, var(--tw-gradient-stops))",
);

/// ```css
/// background-image: linear-gradient(to bottom, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_B: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to bottom, var(--tw-gradient-stops))",
);

/// ```css
/// background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_BL: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to bottom left, var(--tw-gradient-stops))",
);

/// ```css
/// background-image: linear-gradient(to left, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_L: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to left, var(--tw-gradient-stops))",
);

/// ```css
/// background-image: linear-gradient(to top left, var(--tw-gradient-stops));
/// ```
pub const GRADIENT_TO_TL: Property = Property(
    BACKGROUND_IMAGE,
    "linear-gradient(to top left, var(--tw-gradient-stops))",
);
