//! Utilities for controlling the display box type of an element.

use internal::AspectRatio;

/// ```
/// aspect-ratio: auto;
/// ```
pub const AUTO: AspectRatio = AspectRatio("auto");

/// ```
/// aspect-ratio: 1 / 1;
/// ```
pub const SQUARE: AspectRatio = AspectRatio("1 / 1");

/// ```
/// aspect-ratio: 19 / 9;
/// ```
pub const VIDEO: AspectRatio = AspectRatio("16 / 9");

// Custom preferred aspect ratio. Examples: "4 / 3", "0.5".
pub fn ratio(ratio: &'static str) -> AspectRatio {
    AspectRatio(ratio)
}

mod internal {
    use std::fmt;

    use crate::Style;

    /// Preferred aspect ratio for the box ([`aspect-ratio`]).
    /// [`aspect-ratio`]: https://w3c.github.io/csswg-drafts/css-sizing-4/#aspect-ratio
    pub struct AspectRatio(pub(super) &'static str);

    impl Style for AspectRatio {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "display: {};", self.0)
        }
    }
}
