//! Set the margin area on all four sides of an element (`margin`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>

use crate::{Length, Property};

const MARGIN: &str = "margin";

/// ```css
/// margin: 0;
/// ```
pub const ZERO: Property<Length> = Property(MARGIN, Length::Px(0.0));

/// ```css
/// margin: auto;
/// ```
pub const AUTO: Property<Length> = Property(MARGIN, Length::Auto);

/// ```css
/// margin: 1px;
/// ```
pub const PX: Property<Length> = Property(MARGIN, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Rem(x * 0.25))
}

/// ```css
/// margin: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Rem(f32::from(x)))
}

/// ```css
/// margin: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Rem(x))
}

/// ```css
/// margin: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Px(f32::from(x)))
}

/// ```css
/// margin: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Px(x))
}

/// ```css
/// margin: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Percent(f32::from(x)))
}

/// ```css
/// margin: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Percent(x))
}
