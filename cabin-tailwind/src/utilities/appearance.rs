//! Set whether an element is displayed with platform-specific styling (`appearance`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/appearance>

use std::fmt;

use crate::Utility;

/// ```css
/// appearance: none;
/// ```
pub const NONE: Appearance = Appearance("none");

/// ```css
/// appearance: auto;
/// ```
pub const AUTO: Appearance = Appearance("auto");

pub struct Appearance(pub &'static str);

impl Utility for Appearance {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "-webkit-appearance: {};", self.0)?;
        write!(f, "-moz-appearance: {};", self.0)?;
        write!(f, "appearance: {};", self.0)?;
        Ok(())
    }
}
