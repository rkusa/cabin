//! Set the element's maximum width (`max-width`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>

use crate::{Length, Property};

const MAX_WIDTH: &str = "max-width";

/// `max-width: 0;`
pub const ZERO: Property<Length> = Property(MAX_WIDTH, Length::Px(0.0));

/// `max-width: auto;`
pub const AUTO: Property<Length> = Property(MAX_WIDTH, Length::Auto);

/// `max-width: 1px;`
pub const PX: Property<Length> = Property(MAX_WIDTH, Length::Px(1.0));

/// `max-width: 100%;`
pub const FULL: Property<Length> = Property(MAX_WIDTH, Length::Percent(100.0));

/// `max-width: 100vw;`
pub const SCREEN: Property<Length> = Property(MAX_WIDTH, Length::Vw(100));

/// `max-width: min-content;`
pub const MIN: Property<Length> = Property(MAX_WIDTH, Length::MinContent);

/// `max-width: max-content;`
pub const MAX: Property<Length> = Property(MAX_WIDTH, Length::MaxContent);

/// `max-width: fit-content;`
pub const FIT: Property<Length> = Property(MAX_WIDTH, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default): `max-width: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `max-width: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(x * 0.25))
}

/// `max-width: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(f32::from(x)))
}

/// `max-width: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(x))
}

/// `max-width: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Px(f32::from(x)))
}

/// `max-width: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Px(x))
}

/// `max-width: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Percent(f32::from(x)))
}

/// `max-width: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Percent(x))
}

/// `max-width: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Vw(x))
}
