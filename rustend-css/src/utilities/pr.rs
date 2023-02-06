//! Set the padding area on the right of an element (`padding-right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-right>

use crate::{Length, Property};

const PADDING_RIGHT: &str = "padding-right";

/// `padding-right: 0;`
pub const ZERO: Property<Length> = Property(PADDING_RIGHT, Length::Px(0.0));

/// `padding-right: auto;`
pub const AUTO: Property<Length> = Property(PADDING_RIGHT, Length::Auto);

/// `padding-right: 1px;`
pub const PX: Property<Length> = Property(PADDING_RIGHT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `padding-right: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `padding-right: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(x * 0.25))
}

/// `padding-right: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(f32::from(x)))
}

/// `padding-right: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(x))
}

/// `padding-right: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Px(f32::from(x)))
}

/// `padding-right: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Px(x))
}

/// `padding-right: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Percent(f32::from(x)))
}

/// `padding-right: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Percent(x))
}

/// `padding-right: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Vw(x))
}

/// `padding-right: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Vh(x))
}
