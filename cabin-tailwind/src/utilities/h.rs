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
/// height: 100vh;
/// ```
pub const SCREEN: Property<Length> = Property(HEIGHT, Length::Vh(100));

/// ```css
/// height: 100svh;
/// ```
pub const SVH: Property<Length> = Property(HEIGHT, Length::Svh(100));

/// ```css
/// height: 100lvh;
/// ```
pub const LVH: Property<Length> = Property(HEIGHT, Length::Lvh(100));

/// ```css
/// height: 100dvh;
/// ```
pub const DVH: Property<Length> = Property(HEIGHT, Length::Dvh(100));

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
/// height: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(HEIGHT, Length::Em(f32::from(x)))
}

/// ```css
/// height: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Em(x))
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

/// ```css
/// height: {x}svh;
/// ```
pub fn svh(x: u16) -> Property<Length> {
    Property(HEIGHT, Length::Svh(x))
}

/// ```css
/// height: {x}lvh;
/// ```
pub fn lvh(x: u16) -> Property<Length> {
    Property(HEIGHT, Length::Lvh(x))
}

/// ```css
/// height: {x}dvh;
/// ```
pub fn dvh(x: u16) -> Property<Length> {
    Property(HEIGHT, Length::Dvh(x))
}

/// ```css
/// height: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Mm(x))
}

/// ```css
/// height: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Cm(x))
}
