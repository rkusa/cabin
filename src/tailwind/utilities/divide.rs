//! Add a border between child elements.

use std::fmt;

pub use x::{unit as x, unitf as xf};
pub use y::{unit as y, unitf as yf};

use crate::tailwind::{Length, Utility};

pub struct DivideX(pub Length);
pub struct DivideReverseX;
pub struct DivideY(pub Length);
pub struct DivideReverseY;
pub struct DivideStyle(&'static str);
pub struct DivideColor(&'static str);

pub const X: DivideX = x::PX;
pub const Y: DivideY = y::PX;

include!(concat!(env!("OUT_DIR"), "/divide-color.rs"));

/// Set a custom divide border color.
pub fn color(color: &'static str) -> DivideColor {
    DivideColor(color)
}

/// ```css
/// border-style: solid;
/// ```
pub const SOLID: DivideStyle = DivideStyle("solid");

/// ```css
/// border-style: dashed;
/// ```
pub const DASHED: DivideStyle = DivideStyle("dashed");

/// ```css
/// border-style: dotted;
/// ```
pub const DOTTED: DivideStyle = DivideStyle("dotted");

/// ```css
/// border-style: double;
/// ```
pub const DOPUBLE: DivideStyle = DivideStyle("double");

/// ```css
/// border-style: none;
/// ```
pub const NONE: DivideStyle = DivideStyle("none");

pub mod x {
    use super::*;

    pub const ZERO: DivideX = DivideX(Length::Px(0.0));
    pub const PX: DivideX = DivideX(Length::Px(1.0));

    pub const REVERSE: DivideReverseX = DivideReverseX;

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unit(x: i16) -> DivideX {
        DivideX(Length::Rem(f32::from(x) * 0.25))
    }

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unitf(x: f32) -> DivideX {
        DivideX(Length::Rem(x * 0.25))
    }

    pub fn rem(x: i16) -> DivideX {
        DivideX(Length::Rem(f32::from(x)))
    }

    pub fn remf(x: f32) -> DivideX {
        DivideX(Length::Rem(x))
    }

    pub fn em(x: i16) -> DivideX {
        DivideX(Length::Em(f32::from(x)))
    }

    pub fn emf(x: f32) -> DivideX {
        DivideX(Length::Em(x))
    }

    pub fn px(x: i16) -> DivideX {
        DivideX(Length::Px(f32::from(x)))
    }

    pub fn pxf(x: f32) -> DivideX {
        DivideX(Length::Px(x))
    }
}

pub mod y {
    use super::*;

    pub const ZERO: DivideY = DivideY(Length::Px(0.0));
    pub const PX: DivideY = DivideY(Length::Px(1.0));

    pub const REVERSE: DivideReverseY = DivideReverseY;

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unit(x: i16) -> DivideY {
        DivideY(Length::Rem(f32::from(x) * 0.25))
    }

    /// Multiple of `0.25rem` (`4px` by default):
    pub fn unitf(x: f32) -> DivideY {
        DivideY(Length::Rem(x * 0.25))
    }

    pub fn rem(x: i16) -> DivideY {
        DivideY(Length::Rem(f32::from(x)))
    }

    pub fn remf(x: f32) -> DivideY {
        DivideY(Length::Rem(x))
    }

    pub fn em(x: i16) -> DivideY {
        DivideY(Length::Em(f32::from(x)))
    }

    pub fn emf(x: f32) -> DivideY {
        DivideY(Length::Em(x))
    }

    pub fn px(x: i16) -> DivideY {
        DivideY(Length::Px(f32::from(x)))
    }

    pub fn pxf(x: f32) -> DivideY {
        DivideY(Length::Px(x))
    }
}

impl Utility for DivideX {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-divide-x-reverse: 0;\n")?;
        writeln!(
            f,
            "border-right-width: calc({} * var(--tw-divide-x-reverse));",
            self.0
        )?;
        writeln!(
            f,
            "border-left-width: calc({} * calc(1 - var(--tw-divide-x-reverse)));",
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

impl Utility for DivideReverseX {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-divide-x-reverse: 1;\n")
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}

impl Utility for DivideY {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-divide-y-reverse: 0;\n")?;
        writeln!(
            f,
            "border-top-width: calc({} * calc(1 - var(--tw-divide-y-reverse)));",
            self.0
        )?;
        writeln!(
            f,
            "border-bottom-width: calc({} * var(--tw-divide-y-reverse));",
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

impl Utility for DivideReverseY {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str("--tw-divide-y-reverse: 1;\n")
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}

impl Utility for DivideStyle {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "border-style: {};", self.0)
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}

impl Utility for DivideColor {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "border-color: {};", self.0)
    }

    fn selector_suffix(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        f.write_str(" > :not([hidden]) ~ :not([hidden])")
    }

    fn hash_modifier(&self, hasher: &mut dyn std::hash::Hasher) {
        hasher.write(b" > :not([hidden]) ~ :not([hidden])");
    }
}
