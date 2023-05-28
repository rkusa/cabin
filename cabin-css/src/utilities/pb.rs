//! Set the padding area on the bottom of an element (`padding-bottom`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-bottom>

use crate::{Length, Property};

const PADDING_BOTTOM: &str = "padding-bottom";

/// `padding-bottom: 0;`
pub const ZERO: Property<Length> = Property(PADDING_BOTTOM, Length::Px(0.0));

/// `padding-bottom: auto;`
pub const AUTO: Property<Length> = Property(PADDING_BOTTOM, Length::Auto);

/// `padding-bottom: 1px;`
pub const PX: Property<Length> = Property(PADDING_BOTTOM, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `padding-bottom: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `padding-bottom: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Rem(x * 0.25))
}

/// `padding-bottom: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Rem(f32::from(x)))
}

/// `padding-bottom: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Rem(x))
}

/// `padding-bottom: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Px(f32::from(x)))
}

/// `padding-bottom: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Px(x))
}

/// `padding-bottom: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Percent(f32::from(x)))
}

/// `padding-bottom: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Percent(x))
}

/// `padding-bottom: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Vw(x))
}

/// `padding-bottom: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(PADDING_BOTTOM, Length::Vh(x))
}
