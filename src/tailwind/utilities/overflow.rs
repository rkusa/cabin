//! Set how an element handles content that is too large for the container (`overflow`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>

use crate::tailwind::Property;

const OVERFLOW: &str = "overflow";

/// ```css
/// overflow: auto;
/// ```
pub const AUTO: Property = Property(OVERFLOW, "auto");

/// ```css
/// overflow: hidden;
/// ```
pub const HIDDEN: Property = Property(OVERFLOW, "hidden");

/// ```css
/// overflow: clip;
/// ```
pub const CLIP: Property = Property(OVERFLOW, "clip");

/// ```css
/// overflow: visible;
/// ```
pub const VISIBLE: Property = Property(OVERFLOW, "visible");

/// ```css
/// overflow: scroll;
/// ```
pub const SCROLL: Property = Property(OVERFLOW, "scroll");

pub mod x {
    use super::*;

    const OVERFLOW_X: &str = "overflow-x";

    /// ```css
    /// overflow-x: auto;
    /// ```
    pub const AUTO: Property = Property(OVERFLOW_X, "auto");

    /// ```css
    /// overflow-x: hidden;
    /// ```
    pub const HIDDEN: Property = Property(OVERFLOW_X, "hidden");

    /// ```css
    /// overflow-x: clip;
    /// ```
    pub const CLIP: Property = Property(OVERFLOW_X, "clip");

    /// ```css
    /// overflow-x: visible;
    /// ```
    pub const VISIBLE: Property = Property(OVERFLOW_X, "visible");

    /// ```css
    /// overflow-x: scroll;
    /// ```
    pub const SCROLL: Property = Property(OVERFLOW_X, "scroll");
}

pub mod y {
    use super::*;

    const OVERFLOW_Y: &str = "overflow-y";

    /// ```css
    /// overflow-y: auto;
    /// ```
    pub const AUTO: Property = Property(OVERFLOW_Y, "auto");

    /// ```css
    /// overflow-y: hidden;
    /// ```
    pub const HIDDEN: Property = Property(OVERFLOW_Y, "hidden");

    /// ```css
    /// overflow-y: clip;
    /// ```
    pub const CLIP: Property = Property(OVERFLOW_Y, "clip");

    /// ```css
    /// overflow-y: visible;
    /// ```
    pub const VISIBLE: Property = Property(OVERFLOW_Y, "visible");

    /// ```css
    /// overflow-y: scroll;
    /// ```
    pub const SCROLL: Property = Property(OVERFLOW_Y, "scroll");
}
