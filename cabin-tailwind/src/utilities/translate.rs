//! Set translation transforms (`transform`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/transform>

use std::fmt;

pub use x::{unit as x, unitf as xf};
pub use y::{unit as y, unitf as yf};

use crate::{Length, Utility};

pub struct TranslateX(pub Length);
pub struct TranslateY(pub Length);

pub mod x {
    use super::*;

    pub const ZERO: TranslateX = TranslateX(Length::Px(0.0));
    pub const PX: TranslateX = TranslateX(Length::Px(1.0));
    pub const FULL: TranslateX = TranslateX(Length::Percent(100.0));

    /// Multiple of `0.25rem` (`4px` by default):
    /// ```css
    /// transform: translateX({x * 0.25}rem);
    /// ```
    pub fn unit(x: i16) -> TranslateX {
        TranslateX(Length::Rem(f32::from(x) * 0.25))
    }

    /// Multiple of `0.25rem` (`4px` by default):
    /// ```css
    /// transform: translateX({x * 0.25}rem);
    /// ```
    pub fn unitf(x: f32) -> TranslateX {
        TranslateX(Length::Rem(x * 0.25))
    }

    /// ```css
    /// transform: translateX({x}rem);
    /// ```
    pub fn rem(x: i16) -> TranslateX {
        TranslateX(Length::Rem(f32::from(x)))
    }

    /// ```css
    /// transform: translateX({x}rem);
    /// ```
    pub fn remf(x: f32) -> TranslateX {
        TranslateX(Length::Rem(x))
    }

    /// ```css
    /// transform: translateX({x}px);
    /// ```
    pub fn px(x: i16) -> TranslateX {
        TranslateX(Length::Px(f32::from(x)))
    }

    /// ```css
    /// transform: translateX({x}px);
    /// ```
    pub fn pxf(x: f32) -> TranslateX {
        TranslateX(Length::Px(x))
    }

    /// ```css
    /// transform: translateX({x}%);
    /// ```
    pub fn percent(x: i16) -> TranslateX {
        TranslateX(Length::Percent(f32::from(x)))
    }

    /// ```css
    /// transform: translateX({x}%);
    /// ```
    pub fn percentf(x: f32) -> TranslateX {
        TranslateX(Length::Percent(x))
    }
}

pub mod y {
    use super::*;

    pub const ZERO: TranslateY = TranslateY(Length::Px(0.0));
    pub const PX: TranslateY = TranslateY(Length::Px(1.0));
    pub const FULL: TranslateY = TranslateY(Length::Percent(100.0));

    /// Multiple of `0.25rem` (`4px` by default):
    /// ```css
    /// transform: translateY({x * 0.25}rem);
    /// ```
    pub fn unit(x: i16) -> TranslateY {
        TranslateY(Length::Rem(f32::from(x) * 0.25))
    }

    /// Multiple of `0.25rem` (`4px` by default):
    /// ```css
    /// transform: translateY({x * 0.25}rem);
    /// ```
    pub fn unitf(x: f32) -> TranslateY {
        TranslateY(Length::Rem(x * 0.25))
    }

    /// ```css
    /// transform: translateY({x}rem);
    /// ```
    pub fn rem(x: i16) -> TranslateY {
        TranslateY(Length::Rem(f32::from(x)))
    }

    /// ```css
    /// transform: translateY({x}rem);
    /// ```
    pub fn remf(x: f32) -> TranslateY {
        TranslateY(Length::Rem(x))
    }

    /// ```css
    /// transform: translateY({x}px);
    /// ```
    pub fn px(x: i16) -> TranslateY {
        TranslateY(Length::Px(f32::from(x)))
    }

    /// ```css
    /// transform: translateY({x}px);
    /// ```
    pub fn pxf(x: f32) -> TranslateY {
        TranslateY(Length::Px(x))
    }

    /// ```css
    /// transform: translateY({x}%);
    /// ```
    pub fn percent(x: i16) -> TranslateY {
        TranslateY(Length::Percent(f32::from(x)))
    }

    /// ```css
    /// transform: translateY({x}%);
    /// ```
    pub fn percentf(x: f32) -> TranslateY {
        TranslateY(Length::Percent(x))
    }
}

impl Utility for TranslateX {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transform: translateX({});", self.0)?;
        Ok(())
    }
}

impl Utility for TranslateY {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transform: translateY({});", self.0)?;
        Ok(())
    }
}
