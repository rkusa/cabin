//! Set the bottom position of a positioned element (`bottom`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>

use crate::{Length, Property};

const BOTTOM: &str = "bottom";

/// ```css
/// bottom: 0;
/// ```
pub const ZERO: Property<Length> = Property(BOTTOM, Length::Px(0.0));

/// ```css
/// bottom: auto;
/// ```
pub const AUTO: Property<Length> = Property(BOTTOM, Length::Auto);

/// ```css
/// bottom: 1px;
/// ```
pub const PX: Property<Length> = Property(BOTTOM, Length::Px(1.0));

/// ```css
/// bottom: 100%;
/// ```
pub const FULL: Property<Length> = Property(BOTTOM, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// bottom: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(BOTTOM, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// bottom: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(BOTTOM, Length::Rem(x * 0.25))
}

/// ```css
/// bottom: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(BOTTOM, Length::Rem(f32::from(x)))
}

/// ```css
/// bottom: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(BOTTOM, Length::Rem(x))
}
/// ```css
/// bottom: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(BOTTOM, Length::Em(f32::from(x)))
}

/// ```css
/// bottom: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(BOTTOM, Length::Em(x))
}

/// ```css
/// bottom: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(BOTTOM, Length::Px(f32::from(x)))
}

/// ```css
/// bottom: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(BOTTOM, Length::Px(x))
}

/// ```css
/// bottom: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(BOTTOM, Length::Percent(f32::from(x)))
}

/// ```css
/// bottom: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(BOTTOM, Length::Percent(x))
}

/// ```css
/// bottom: {x}vh;
/// ```
pub fn vh(x: u16) -> Property<Length> {
    Property(BOTTOM, Length::Vh(x))
}

/// ```css
/// bottom: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(BOTTOM, Length::Mm(x))
}

/// ```css
/// bottom: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(BOTTOM, Length::Cm(x))
}
