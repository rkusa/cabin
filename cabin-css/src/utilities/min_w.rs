//! Set the element's minimum width (`min-width`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>

use crate::{Length, Property};

const MIN_WIDTH: &str = "min-width";

/// `min-width: 0;`
pub const ZERO: Property<Length> = Property(MIN_WIDTH, Length::Px(0.0));

/// `min-width: auto;`
pub const AUTO: Property<Length> = Property(MIN_WIDTH, Length::Auto);

/// `min-width: 1px;`
pub const PX: Property<Length> = Property(MIN_WIDTH, Length::Px(1.0));

/// `min-width: 100%;`
pub const FULL: Property<Length> = Property(MIN_WIDTH, Length::Percent(100.0));

/// `min-width: 100vw;`
pub const SCREEN: Property<Length> = Property(MIN_WIDTH, Length::Vw(100));

/// `min-width: min-content;`
pub const MIN: Property<Length> = Property(MIN_WIDTH, Length::MinContent);

/// `min-width: max-content;`
pub const MAX: Property<Length> = Property(MIN_WIDTH, Length::MaxContent);

/// `min-width: fit-content;`
pub const FIT: Property<Length> = Property(MIN_WIDTH, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default): `min-width: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `min-width: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(x * 0.25))
}

/// `min-width: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(f32::from(x)))
}

/// `min-width: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(x))
}

/// `min-width: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Px(f32::from(x)))
}

/// `min-width: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Px(x))
}

/// `min-width: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Percent(f32::from(x)))
}

/// `min-width: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Percent(x))
}

/// `min-width: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Vw(x))
}
