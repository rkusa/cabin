//! Set rotation transforms (`transform`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/transform>

use std::fmt;

use crate::Utility;

pub struct Rotate(pub i16);

/// ```css
/// transform: rotate({x}deg);
/// ```
pub fn deg(x: i16) -> Rotate {
    Rotate(x)
}

impl Utility for Rotate {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transform: rotate({}deg);", self.0)?;
        Ok(())
    }
}
