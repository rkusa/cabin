//! Add space between child elements.

use std::fmt;

pub use x::{unit as x, unitf as xf};
pub use y::{unit as y, unitf as yf};

use crate::{Length, Utility};

pub struct SpaceX(pub Length);
pub struct SpaceReverseX;
pub struct SpaceY(pub Length);
pub struct SpaceReverseY;

pub mod x {
    use super::*;

    pub const ZERO: SpaceX = SpaceX(Length::Px(0.0));
    pub const PX: SpaceX = SpaceX(Length::Px(1.0));

    pub const REVERSE: SpaceReverseX = SpaceReverseX;

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unit(x: i16) -> SpaceX {
        SpaceX(Length::Rem(f32::from(x) * 0.25))
    }

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unitf(x: f32) -> SpaceX {
        SpaceX(Length::Rem(x * 0.25))
    }

    pub fn rem(x: i16) -> SpaceX {
        SpaceX(Length::Rem(f32::from(x)))
    }

    pub fn remf(x: f32) -> SpaceX {
        SpaceX(Length::Rem(x))
    }

    pub fn px(x: i16) -> SpaceX {
        SpaceX(Length::Px(f32::from(x)))
    }

    pub fn pxf(x: f32) -> SpaceX {
        SpaceX(Length::Px(x))
    }
}

pub mod y {
    use super::*;

    pub const ZERO: SpaceY = SpaceY(Length::Px(0.0));
    pub const PX: SpaceY = SpaceY(Length::Px(1.0));

    pub const REVERSE: SpaceReverseY = SpaceReverseY;

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unit(x: i16) -> SpaceY {
        SpaceY(Length::Rem(f32::from(x) * 0.25))
    }

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unitf(x: f32) -> SpaceY {
        SpaceY(Length::Rem(x * 0.25))
    }

    pub fn rem(x: i16) -> SpaceY {
        SpaceY(Length::Rem(f32::from(x)))
    }

    pub fn remf(x: f32) -> SpaceY {
        SpaceY(Length::Rem(x))
    }

    pub fn px(x: i16) -> SpaceY {
        SpaceY(Length::Px(f32::from(x)))
    }

    pub fn pxf(x: f32) -> SpaceY {
        SpaceY(Length::Px(x))
    }
}

impl Utility for SpaceX {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-space-x-reverse: 0;")?;
        write!(
            f,
            "margin-inline-end: calc({} * var(--tw-space-x-reverse));",
            self.0
        )?;
        write!(
            f,
            "margin-inline-start: calc({} * calc(1 - var(--tw-space-x-reverse)));",
            self.0
        )?;
        Ok(())
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}

impl Utility for SpaceReverseX {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-space-x-reverse: 1;")
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}

impl Utility for SpaceY {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-space-y-reverse: 0;")?;
        write!(
            f,
            "margin-top: calc({} * calc(1 - var(--tw-space-y-reverse)));",
            self.0
        )?;
        write!(
            f,
            "margin-bottom: calc({} * var(--tw-space-y-reverse));",
            self.0
        )?;
        Ok(())
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}

impl Utility for SpaceReverseY {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-space-y-reverse: 1;")
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}
