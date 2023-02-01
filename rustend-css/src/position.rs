//! Utilities for controlling the display box type of an element.

use internal::Position;

/// ```
/// position: static;
/// ```
pub const STATIC: Position = Position("static");

/// ```
/// position: relative;
/// ```
pub const RELATIVE: Position = Position("relative");

/// ```
/// position: absolute;
/// ```
pub const ABSOLUTE: Position = Position("absolute");

/// ```
/// position: sticky;
/// ```
pub const STICKY: Position = Position("sticky");

/// ```
/// position: fixed;
/// ```
pub const FIXED: Position = Position("fixed");

mod internal {
    use std::fmt;

    use crate::Style;

    /// Positioning schemes used to calculate the position of a box ([`position`]).
    /// [`position`]: https://w3c.github.io/csswg-drafts/css-position/#position-property
    pub struct Position(pub(super) &'static str);

    impl Style for Position {
        fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "position: {};", self.0)
        }
    }
}
