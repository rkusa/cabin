//! Set the top position of a positioned element (`top`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/top>

use crate::{Length, Property};

const TOP: &str = "top";

/// ```css
/// top: 0;
/// ```
pub const ZERO: Property<Length> = Property(TOP, Length::Px(0.0));

/// ```css
/// top: auto;
/// ```
pub const AUTO: Property<Length> = Property(TOP, Length::Auto);

/// ```css
/// top: 1px;
/// ```
pub const PX: Property<Length> = Property(TOP, Length::Px(1.0));

/// ```css
/// top: 100%;
/// ```
pub const FULL: Property<Length> = Property(TOP, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// top: {x * 0.25}rem;
/// ```
pub fn unit(x: u16) -> Property<Length> {
    Property(TOP, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// top: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(TOP, Length::Rem(x * 0.25))
}

/// ```css
/// top: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(TOP, Length::Rem(f32::from(x)))
}

/// ```css
/// top: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(TOP, Length::Rem(x))
}

/// ```css
/// top: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(TOP, Length::Px(f32::from(x)))
}

/// ```css
/// top: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(TOP, Length::Px(x))
}

/// ```css
/// top: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(TOP, Length::Percent(f32::from(x)))
}

/// ```css
/// top: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(TOP, Length::Percent(x))
}

/// ```css
/// top: {x}vh;
/// ```
pub fn vh(x: u16) -> Property<Length> {
    Property(TOP, Length::Vh(x))
}
