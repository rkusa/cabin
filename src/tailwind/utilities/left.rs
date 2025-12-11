//! Set the left position of a positioned element (`left`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/left>

use crate::tailwind::{Length, Property};

const LEFT: &str = "left";

/// ```css
/// left: 0;
/// ```
pub const ZERO: Property<Length> = Property(LEFT, Length::Px(0.0));

/// ```css
/// left: auto;
/// ```
pub const AUTO: Property<Length> = Property(LEFT, Length::Auto);

/// ```css
/// left: 1px;
/// ```
pub const PX: Property<Length> = Property(LEFT, Length::Px(1.0));

/// ```css
/// left: 100%;
/// ```
pub const FULL: Property<Length> = Property(LEFT, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// left: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(LEFT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// left: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(LEFT, Length::Rem(x * 0.25))
}

/// ```css
/// left: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(LEFT, Length::Rem(f32::from(x)))
}

/// ```css
/// left: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(LEFT, Length::Rem(x))
}

/// ```css
/// left: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(LEFT, Length::Em(f32::from(x)))
}

/// ```css
/// left: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(LEFT, Length::Em(x))
}

/// ```css
/// left: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(LEFT, Length::Px(f32::from(x)))
}

/// ```css
/// left: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(LEFT, Length::Px(x))
}

/// ```css
/// left: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(LEFT, Length::Percent(f32::from(x)))
}

/// ```css
/// left: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(LEFT, Length::Percent(x))
}

/// ```css
/// left: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(LEFT, Length::Vw(x))
}

/// ```css
/// left: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(LEFT, Length::Mm(x))
}

/// ```css
/// left: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(LEFT, Length::Cm(x))
}
