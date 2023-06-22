//! Set the element's minimum height (`min-height`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/min-height>

use crate::{Length, Property};

const MIN_HEIGHT: &str = "min-height";

/// `min-height: 0;`
pub const ZERO: Property<Length> = Property(MIN_HEIGHT, Length::Px(0.0));

/// `min-height: auto;`
pub const AUTO: Property<Length> = Property(MIN_HEIGHT, Length::Auto);

/// `min-height: 1px;`
pub const PX: Property<Length> = Property(MIN_HEIGHT, Length::Px(1.0));

/// `min-height: 100%;`
pub const FULL: Property<Length> = Property(MIN_HEIGHT, Length::Percent(100.0));

/// `min-height: 100vw;`
pub const SCREEN: Property<Length> = Property(MIN_HEIGHT, Length::Vw(100));

/// `min-height: min-content;`
pub const MIN: Property<Length> = Property(MIN_HEIGHT, Length::MinContent);

/// `min-height: max-content;`
pub const MAX: Property<Length> = Property(MIN_HEIGHT, Length::MaxContent);

/// `min-height: fit-content;`
pub const FIT: Property<Length> = Property(MIN_HEIGHT, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default): `min-height: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `min-height: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Rem(x * 0.25))
}

/// `min-height: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Rem(f32::from(x)))
}

/// `min-height: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Rem(x))
}

/// `min-height: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Px(f32::from(x)))
}

/// `min-height: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Px(x))
}

/// `min-height: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Percent(f32::from(x)))
}

/// `min-height: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Percent(x))
}

/// `min-height: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Vw(x))
}

/// `min-height: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(MIN_HEIGHT, Length::Vh(x))
}
