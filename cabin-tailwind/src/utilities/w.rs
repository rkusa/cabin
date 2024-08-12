//! Set the element's width (`width`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/width>

use crate::{Length, Property};

const WIDTH: &str = "width";

/// ```css
/// width: 0;
/// ```
pub const ZERO: Property<Length> = Property(WIDTH, Length::Px(0.0));

/// ```css
/// width: auto;
/// ```
pub const AUTO: Property<Length> = Property(WIDTH, Length::Auto);

/// ```css
/// width: 1px;
/// ```
pub const PX: Property<Length> = Property(WIDTH, Length::Px(1.0));

/// ```css
/// width: 100%;
/// ```
pub const FULL: Property<Length> = Property(WIDTH, Length::Percent(100.0));

/// ```css
/// width: 100vw;
/// ```
pub const SCREEN: Property<Length> = Property(WIDTH, Length::Vw(100));

/// ```css
/// width: 100svw;
/// ```
pub const SVH: Property<Length> = Property(WIDTH, Length::Svw(100));

/// ```css
/// width: 100lvw;
/// ```
pub const LVH: Property<Length> = Property(WIDTH, Length::Lvw(100));

/// ```css
/// width: 100dvw;
/// ```
pub const DVH: Property<Length> = Property(WIDTH, Length::Dvw(100));

/// ```css
/// width: min-content;
/// ```
pub const MIN: Property<Length> = Property(WIDTH, Length::MinContent);

/// ```css
/// width: max-content;
/// ```
pub const MAX: Property<Length> = Property(WIDTH, Length::MaxContent);

/// ```css
/// width: fit-content;
/// ```
pub const FIT: Property<Length> = Property(WIDTH, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// width: {x * 0.25}rem;
/// ```
pub fn unit(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// width: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Rem(x * 0.25))
}

/// ```css
/// width: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(WIDTH, Length::Rem(f32::from(x)))
}

/// ```css
/// width: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Rem(x))
}

/// ```css
/// width: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// width: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Px(x))
}

/// ```css
/// width: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(WIDTH, Length::Percent(f32::from(x)))
}

/// ```css
/// width: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Percent(x))
}

/// ```css
/// width: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Vw(x))
}

/// ```css
/// width: {x}svw;
/// ```
pub fn svw(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Svw(x))
}

/// ```css
/// width: {x}lvw;
/// ```
pub fn lvw(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Lvw(x))
}

/// ```css
/// width: {x}dvw;
/// ```
pub fn dvw(x: u16) -> Property<Length> {
    Property(WIDTH, Length::Dvw(x))
}

/// ```css
/// width: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Mm(x))
}

/// ```css
/// width: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(WIDTH, Length::Cm(x))
}
