//! Set the padding area on the top of an element (`padding-top`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-top>

use crate::{Length, Property};

const PADDING_TOP: &str = "padding-top";

/// `padding-top: 0;`
pub const ZERO: Property<Length> = Property(PADDING_TOP, Length::Px(0.0));

/// `padding-top: auto;`
pub const AUTO: Property<Length> = Property(PADDING_TOP, Length::Auto);

/// `padding-top: 1px;`
pub const PX: Property<Length> = Property(PADDING_TOP, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `padding-top: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(PADDING_TOP, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `padding-top: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(PADDING_TOP, Length::Rem(x * 0.25))
}

/// `padding-top: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(PADDING_TOP, Length::Rem(f32::from(x)))
}

/// `padding-top: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(PADDING_TOP, Length::Rem(x))
}

/// `padding-top: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(PADDING_TOP, Length::Px(f32::from(x)))
}

/// `padding-top: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(PADDING_TOP, Length::Px(x))
}

/// `padding-top: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(PADDING_TOP, Length::Percent(f32::from(x)))
}

/// `padding-top: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(PADDING_TOP, Length::Percent(x))
}

/// `padding-top: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(PADDING_TOP, Length::Vw(x))
}

/// `padding-top: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(PADDING_TOP, Length::Vh(x))
}
