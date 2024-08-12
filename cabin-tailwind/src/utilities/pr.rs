//! Set the padding area on the right of an element (`padding-right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-right>

use crate::{Length, Property};

const PADDING_RIGHT: &str = "padding-right";

/// ```css
/// padding-right: 0;
/// ```
pub const ZERO: Property<Length> = Property(PADDING_RIGHT, Length::Px(0.0));

/// ```css
/// padding-right: auto;
/// ```
pub const AUTO: Property<Length> = Property(PADDING_RIGHT, Length::Auto);

/// ```css
/// padding-right: 1px;
/// ```
pub const PX: Property<Length> = Property(PADDING_RIGHT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding-right: {x * 0.25}rem;
/// ```
pub fn unit(x: u16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding-right: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// padding-right: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// padding-right: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Rem(x))
}

/// ```css
/// padding-right: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Em(f32::from(x)))
}

/// ```css
/// padding-right: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Em(x))
}

/// ```css
/// padding-right: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// padding-right: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Px(x))
}

/// ```css
/// padding-right: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Percent(f32::from(x)))
}

/// ```css
/// padding-right: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Percent(x))
}

/// ```css
/// padding-right: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Vw(x))
}

/// ```css
/// padding-right: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Mm(x))
}

/// ```css
/// padding-right: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(PADDING_RIGHT, Length::Cm(x))
}
