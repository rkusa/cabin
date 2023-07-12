//! Set the element's maximum height (`max-height`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/max-height>

use crate::{Length, Property};

const MAX_HEIGHT: &str = "max-height";

/// ```css
/// max-height: 0;
/// ```
pub const ZERO: Property<Length> = Property(MAX_HEIGHT, Length::Px(0.0));

/// ```css
/// max-height: auto;
/// ```
pub const AUTO: Property<Length> = Property(MAX_HEIGHT, Length::Auto);

/// ```css
/// max-height: 1px;
/// ```
pub const PX: Property<Length> = Property(MAX_HEIGHT, Length::Px(1.0));

/// ```css
/// max-height: 100%;
/// ```
pub const FULL: Property<Length> = Property(MAX_HEIGHT, Length::Percent(100.0));

/// ```css
/// max-height: 100vw;
/// ```
pub const SCREEN: Property<Length> = Property(MAX_HEIGHT, Length::Vh(100));

/// ```css
/// max-height: min-content;
/// ```
pub const MIN: Property<Length> = Property(MAX_HEIGHT, Length::MinContent);

/// ```css
/// max-height: max-content;
/// ```
pub const MAX: Property<Length> = Property(MAX_HEIGHT, Length::MaxContent);

/// ```css
/// max-height: fit-content;
/// ```
pub const FIT: Property<Length> = Property(MAX_HEIGHT, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// max-height: {x * 0.25}rem;
/// ```
pub fn unit(x: u16) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// max-height: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// max-height: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// max-height: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Rem(x))
}

/// ```css
/// max-height: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// max-height: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Px(x))
}

/// ```css
/// max-height: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Percent(f32::from(x)))
}

/// ```css
/// max-height: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Percent(x))
}

/// ```css
/// max-height: {x}vh;
/// ```
pub fn vh(x: u16) -> Property<Length> {
    Property(MAX_HEIGHT, Length::Vh(x))
}
