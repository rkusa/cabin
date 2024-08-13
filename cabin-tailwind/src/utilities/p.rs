//! Set the padding area on all four sides of an element (`padding`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding>

use crate::{Length, Property};

const PADDING: &str = "padding";

/// ```css
/// padding: 0;
/// ```
pub const ZERO: Property<Length> = Property(PADDING, Length::Px(0.0));

/// ```css
/// padding: auto;
/// ```
pub const AUTO: Property<Length> = Property(PADDING, Length::Auto);

/// ```css
/// padding: 1px;
/// ```
pub const PX: Property<Length> = Property(PADDING, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(PADDING, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(PADDING, Length::Rem(x * 0.25))
}

/// ```css
/// padding: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(PADDING, Length::Rem(f32::from(x)))
}

/// ```css
/// padding: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(PADDING, Length::Rem(x))
}

/// ```css
/// padding: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(PADDING, Length::Em(f32::from(x)))
}

/// ```css
/// padding: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(PADDING, Length::Em(x))
}

/// ```css
/// padding: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(PADDING, Length::Px(f32::from(x)))
}

/// ```css
/// padding: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(PADDING, Length::Px(x))
}

/// ```css
/// padding: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(PADDING, Length::Percent(f32::from(x)))
}

/// ```css
/// padding: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(PADDING, Length::Percent(x))
}

/// ```css
/// padding: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(PADDING, Length::Mm(x))
}

/// ```css
/// padding: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(PADDING, Length::Cm(x))
}
