//! Rounds the corners of an element's bottom left outer border edge (`border-bottom-left-radius`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>

use crate::{Length, Property};

const BORDER_BOTTOM_LEFT_RADIUS: &str = "border-bottom-left-radius";

include!(concat!(env!("OUT_DIR"), "/rounded-bl.rs"));

/// ```css
/// border-bottom-left-radius: 0;
/// ```
pub const NONE: Property<Length> = Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Px(0.0));

/// ```css
/// border-bottom-left-radius: 0.25rem;
/// ```
pub const DEFAULT: Property<Length> = Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Rem(0.25));

/// ```css
/// border-bottom-left-radius: 9999px;
/// ```
pub const FULL: Property<Length> = Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Px(9999.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// border-bottom-left-radius: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// border-bottom-left-radius: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Rem(x * 0.25))
}

/// ```css
/// border-bottom-left-radius: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Rem(f32::from(x)))
}

/// ```css
/// border-bottom-left-radius: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Rem(x))
}

/// ```css
/// border-bottom-left-radius: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Px(f32::from(x)))
}

/// ```css
/// border-bottom-left-radius: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Px(x))
}

/// ```css
/// border-bottom-left-radius: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Percent(f32::from(x)))
}

/// ```css
/// border-bottom-left-radius: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(BORDER_BOTTOM_LEFT_RADIUS, Length::Percent(x))
}
