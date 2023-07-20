//! Utilities for clamping text to a specific number of lines.

use std::fmt;

use crate::Utility;

/// ```css
/// overflow: visible;
/// display: block;
/// -webkit-box-orient: horizontal;
/// -webkit-line-clamp: {n};
/// ```
pub fn clamp(n: usize) -> LineClamp {
    LineClamp(n)
}

pub struct LineClamp(pub usize);

impl Utility for LineClamp {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("overflow: hidden;")?;
        f.write_str("display: -webkit-box;")?;
        f.write_str("-webkit-box-orient: vertical;")?;
        write!(f, "-webkit-line-clamp: {};", self.0)?;
        Ok(())
    }
}

pub mod clamp {
    use super::*;

    /// Undo a previously applied line clamp utility.
    /// ```css
    /// overflow: visible;
    /// display: block;
    /// -webkit-box-orient: horizontal;
    /// -webkit-line-clamp: none;
    /// ```
    pub const NONE: NoLineClamp = NoLineClamp;

    pub struct NoLineClamp;

    impl Utility for NoLineClamp {
        fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
            f.write_str("line-clamp-none	overflow: visible;")?;
            f.write_str("display: block;")?;
            f.write_str("-webkit-box-orient: horizontal;")?;
            f.write_str("-webkit-line-clamp: none;")?;
            Ok(())
        }
    }
}
