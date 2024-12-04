//! Utilities for controlling an element's border.

use crate::{Length, Property, PropertyTwice};

const BORDER_STYLE: &str = "border-style";
const BORDER_WIDTH: &str = "border-width";
const BORDER_LEFT_WIDTH: &str = "border-left-width";
const BORDER_RIGHT_WIDTH: &str = "border-right-width";
const BORDER_TOP_WIDTH: &str = "border-top-width";
const BORDER_BOTTOM_WIDTH: &str = "border-bottom-width";
const BORDER_INLINE_START_WIDTH: &str = "border-inline-start-width";
const BORDER_INLINE_END_WIDTH: &str = "border-inline-end-width";
const BORDER_COLOR: &str = "border-color";
const BORDER_LEFT_COLOR: &str = "border-left-color";
const BORDER_RIGHT_COLOR: &str = "border-right-color";
const BORDER_TOP_COLOR: &str = "border-top-color";
const BORDER_BOTTOM_COLOR: &str = "border-bottom-color";
const BORDER_INLINE_START_COLOR: &str = "border-inline-start-color";
const BORDER_INLINE_END_COLOR: &str = "border-inline-end-color";

include!(concat!(env!("OUT_DIR"), "/border-color.rs"));

/// Set a custom border color.
pub fn color(color: &'static str) -> Property {
    Property(BORDER_COLOR, color)
}

/// Remove an existing border style.
/// ```css
/// border-style: none;
/// ```
pub const NONE: Property = Property(BORDER_STYLE, "none");

/// ```css
/// border-style: solid;
/// ```
pub const SOLID: Property = Property(BORDER_STYLE, "solid");

/// ```css
/// border-style: dashed;
/// ```
pub const DASHED: Property = Property(BORDER_STYLE, "dashed");

/// ```css
/// border-style: dotted;
/// ```
pub const DOTTED: Property = Property(BORDER_STYLE, "dotted");

/// ```css
/// border-style: double;
/// ```
pub const DOUBLE: Property = Property(BORDER_STYLE, "double");

/// ```css
/// border-style: hidden;
/// ```
pub const HIDDEN: Property = Property(BORDER_STYLE, "hidden");

/// ```css
/// border-width: 1px;
/// ```
pub const PX: Property<Length> = Property(BORDER_WIDTH, Length::Px(1.0));

/// ```css
/// border-left-width: 1px;
/// border-right-width: 1px;
/// ```
pub const X: PropertyTwice<Length> =
    PropertyTwice(BORDER_LEFT_WIDTH, BORDER_RIGHT_WIDTH, Length::Px(1.0));

/// ```css
/// border-top-width: 1px;
/// border-bottom-width: 1px;
/// ```
pub const Y: PropertyTwice<Length> =
    PropertyTwice(BORDER_TOP_WIDTH, BORDER_BOTTOM_WIDTH, Length::Px(1.0));

/// ```css
/// border-inline-start-width: 1px;
/// ```
pub const S: Property<Length> = Property(BORDER_INLINE_START_WIDTH, Length::Px(1.0));

/// ```css
/// border-inline-end-width: 1px;
/// ```
pub const E: Property<Length> = Property(BORDER_INLINE_END_WIDTH, Length::Px(1.0));

/// ```css
/// border-top-width: 1px;
/// ```
pub const T: Property<Length> = Property(BORDER_TOP_WIDTH, Length::Px(1.0));

/// ```css
/// border-bottom-width: 1px;
/// ```
pub const B: Property<Length> = Property(BORDER_BOTTOM_WIDTH, Length::Px(1.0));

/// ```css
/// border-left-width: 1px;
/// ```
pub const L: Property<Length> = Property(BORDER_LEFT_WIDTH, Length::Px(1.0));

/// ```css
/// border-right-width: 1px;
/// ```
pub const R: Property<Length> = Property(BORDER_RIGHT_WIDTH, Length::Px(1.0));

/// ```css
/// border-width: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(BORDER_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// border-left-width: {x}px;
/// border-right-width: {x}px;
/// ```
pub fn x(x: i16) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_LEFT_WIDTH,
        BORDER_RIGHT_WIDTH,
        Length::Px(f32::from(x)),
    )
}

/// ```css
/// border-top-width: {x}px;
/// border-bottom-width: {x}px;
/// ```
pub fn y(x: i16) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_WIDTH,
        BORDER_BOTTOM_WIDTH,
        Length::Px(f32::from(x)),
    )
}

/// ```css
/// border-inline-start-width: {x}px;
/// ```
pub fn s(x: i16) -> Property<Length> {
    Property(BORDER_INLINE_START_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// border-inline-end-width: {x}px;
/// ```
pub fn e(x: i16) -> Property<Length> {
    Property(BORDER_INLINE_END_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// border-top-width: {x}px;
/// ```
pub fn t(x: i16) -> Property<Length, 1> {
    Property(BORDER_TOP_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// border-bottom-width: {x}px;
/// ```
pub fn b(x: i16) -> Property<Length, 1> {
    Property(BORDER_BOTTOM_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// border-left-width: {x}px;
/// ```
pub fn l(x: i16) -> Property<Length, 1> {
    Property(BORDER_LEFT_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// border-right-width: {x}px;
/// ```
pub fn r(x: i16) -> Property<Length, 1> {
    Property(BORDER_RIGHT_WIDTH, Length::Px(f32::from(x)))
}

pub mod x {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-x-color.rs"));
}

pub mod y {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-y-color.rs"));
}

pub mod s {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-s-color.rs"));
}

pub mod e {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-e-color.rs"));
}

pub mod t {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-t-color.rs"));
}

pub mod b {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-b-color.rs"));
}

pub mod l {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-l-color.rs"));
}

pub mod r {
    use super::*;
    include!(concat!(env!("OUT_DIR"), "/border-r-color.rs"));
}
