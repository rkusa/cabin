//! Set the padding area on the left of an element (`padding-left`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-left>

use crate::{Length, Property};

const PADDING_LEFT: &str = "padding-left";

/// `padding-left: 0;`
pub const ZERO: Property<Length> = Property(PADDING_LEFT, Length::Px(0.0));

/// `padding-left: auto;`
pub const AUTO: Property<Length> = Property(PADDING_LEFT, Length::Auto);

/// `padding-left: 1px;`
pub const PX: Property<Length> = Property(PADDING_LEFT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `padding-left: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(PADDING_LEFT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `padding-left: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(PADDING_LEFT, Length::Rem(x * 0.25))
}

/// `padding-left: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(PADDING_LEFT, Length::Rem(f32::from(x)))
}

/// `padding-left: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(PADDING_LEFT, Length::Rem(x))
}

/// `padding-left: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(PADDING_LEFT, Length::Px(f32::from(x)))
}

/// `padding-left: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(PADDING_LEFT, Length::Px(x))
}

/// `padding-left: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(PADDING_LEFT, Length::Percent(f32::from(x)))
}

/// `padding-left: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(PADDING_LEFT, Length::Percent(x))
}

/// `padding-left: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(PADDING_LEFT, Length::Vw(x))
}
