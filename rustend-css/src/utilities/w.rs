//! Set the element's width (`width`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/width>

use crate::{Length, Property};

const WIDTH: &str = "width";

/// `width: 0;`
pub const ZERO: Property<Length> = Property(WIDTH, Length::Px(0.0));

/// `width: auto;`
pub const AUTO: Property<Length> = Property(WIDTH, Length::Auto);

/// `width: 1px;`
pub const PX: Property<Length> = Property(WIDTH, Length::Px(1.0));

/// `width: 100%;`
pub const FULL: Property<Length> = Property(WIDTH, Length::Percent(100.0));

/// `width: 100vw;`
pub const SCREEN: Property<Length> = Property(WIDTH, Length::Vw(100));

/// `width: min-content;`
pub const MIN: Property<Length> = Property(WIDTH, Length::MinContent);

/// `width: max-content;`
pub const MAX: Property<Length> = Property(WIDTH, Length::MaxContent);

/// `width: fit-content;`
pub const FIT: Property<Length> = Property(WIDTH, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default): `width: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `width: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Rem(x * 0.25))
}

/// `width: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(WIDTH, Length::Rem(f32::from(x)))
}

/// `width: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Rem(x))
}

/// `width: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(WIDTH, Length::Px(f32::from(x)))
}

/// `width: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Px(x))
}

/// `width: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(WIDTH, Length::Percent(f32::from(x)))
}

/// `width: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Percent(x))
}

/// `width: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Vw(x))
}

/// `width: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Vh(x))
}
