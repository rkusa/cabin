//! Set the element's line height (`line-height`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>

use crate::{Length, Property};

const LINE_HEIGHT: &str = "line-height";

// NOTE: Should take precedence over text::LG etc.

/// ```css
/// line-height: 1;
/// ```
pub const NONE: Property<f64, 1> = Property(LINE_HEIGHT, 1.0);

/// ```css
/// line-height: 1.25;
/// ```
pub const TIGHT: Property<f64, 1> = Property(LINE_HEIGHT, 1.25);

/// ```css
/// line-height: 1.375;
/// ```
pub const SNUG: Property<f64, 1> = Property(LINE_HEIGHT, 1.375);

/// ```css
/// line-height: 1.5;
/// ```
pub const NORMAL: Property<f64, 1> = Property(LINE_HEIGHT, 1.5);

/// ```css
/// line-height: 1.625;
/// ```
pub const RELAXED: Property<f64, 1> = Property(LINE_HEIGHT, 1.625);

/// ```css
/// line-height: 2;
/// ```
pub const LOOSE: Property<f64, 1> = Property(LINE_HEIGHT, 2.0);

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// line-height: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// line-height: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// line-height: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// line-height: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Rem(x))
}

/// ```css
/// line-height: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Em(f32::from(x)))
}

/// ```css
/// line-height: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Em(x))
}

/// ```css
/// line-height: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// line-height: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length, 1> {
    Property(LINE_HEIGHT, Length::Px(x))
}
