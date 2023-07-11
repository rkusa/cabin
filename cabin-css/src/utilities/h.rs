//! Set the element's height (`height`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/height>

use crate::{Length, Property};

const HEIGHT: &str = "height";

/// ```css
/// height: 0;
/// ```
pub const ZERO: Property<Length> = Property(HEIGHT, Length::Px(0.0));

/// ```css
/// height: auto;
/// ```
pub const AUTO: Property<Length> = Property(HEIGHT, Length::Auto);

/// ```css
/// height: 1px;
/// ```
pub const PX: Property<Length> = Property(HEIGHT, Length::Px(1.0));

/// ```css
/// height: 100%;
/// ```
pub const FULL: Property<Length> = Property(HEIGHT, Length::Percent(100.0));

/// ```css
/// height: 100vw;
/// ```
pub const SCREEN: Property<Length> = Property(HEIGHT, Length::Vh(100));

/// ```css
/// height: min-content;
/// ```
pub const MIN: Property<Length> = Property(HEIGHT, Length::MinContent);

/// ```css
/// height: max-content;
/// ```
pub const MAX: Property<Length> = Property(HEIGHT, Length::MaxContent);

/// ```css
/// height: fit-content;
/// ```
pub const FIT: Property<Length> = Property(HEIGHT, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// height: {x * 0.25}rem;
/// ```
pub fn unit(x: u16) -> Property<Length> {
    Property(HEIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// height: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// height: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(HEIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// height: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Rem(x))
}

/// ```css
/// height: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(HEIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// height: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Px(x))
}

/// ```css
/// height: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(HEIGHT, Length::Percent(f32::from(x)))
}

/// ```css
/// height: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Percent(x))
}

/// ```css
/// height: {x}vh;
/// ```
pub fn vh(x: u16) -> Property<Length> {
    Property(HEIGHT, Length::Vh(x))
}
