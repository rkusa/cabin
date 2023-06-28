//! Set the right position of a positioned element (`right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/right>

use crate::{Length, Property};

const RIGHT: &str = "right";

/// ```css
/// right: 0;
/// ```
pub const ZERO: Property<Length> = Property(RIGHT, Length::Px(0.0));

/// ```css
/// right: auto;
/// ```
pub const AUTO: Property<Length> = Property(RIGHT, Length::Auto);

/// ```css
/// right: 1px;
/// ```
pub const PX: Property<Length> = Property(RIGHT, Length::Px(1.0));

/// ```css
/// right: 100%;
/// ```
pub const FULL: Property<Length> = Property(RIGHT, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// right: {x * 0.25}rem;
/// ```
pub fn unit(x: u16) -> Property<Length> {
    Property(RIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// right: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(RIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// right: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(RIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// right: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(RIGHT, Length::Rem(x))
}

/// ```css
/// right: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(RIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// right: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(RIGHT, Length::Px(x))
}

/// ```css
/// right: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(RIGHT, Length::Percent(f32::from(x)))
}

/// ```css
/// right: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(RIGHT, Length::Percent(x))
}

/// ```css
/// right: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(RIGHT, Length::Vw(x))
}
