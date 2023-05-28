//! Set the margin area on the right of an element (`margin-right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-right>

use crate::{Length, Property};

const MARGIN_RIGHT: &str = "margin-right";

/// `margin-right: 0;`
pub const ZERO: Property<Length> = Property(MARGIN_RIGHT, Length::Px(0.0));

/// `margin-right: auto;`
pub const AUTO: Property<Length> = Property(MARGIN_RIGHT, Length::Auto);

/// `margin-right: 1px;`
pub const PX: Property<Length> = Property(MARGIN_RIGHT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `margin-right: {x * 0.25}rem;`
pub fn unit(x: i16) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `margin-right: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Rem(x * 0.25))
}

/// `margin-right: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Rem(f32::from(x)))
}

/// `margin-right: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Rem(x))
}

/// `margin-right: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Px(f32::from(x)))
}

/// `margin-right: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Px(x))
}

/// `margin-right: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Percent(f32::from(x)))
}

/// `margin-right: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Percent(x))
}

/// `margin-right: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Vw(x))
}

/// `margin-right: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(MARGIN_RIGHT, Length::Vh(x))
}
