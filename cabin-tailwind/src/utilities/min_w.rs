//! Set the element's minimum width (`min-width`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>

use crate::{Length, Property};

const MIN_WIDTH: &str = "min-width";

/// ```css
/// min-width: 0;
/// ```
pub const ZERO: Property<Length> = Property(MIN_WIDTH, Length::Px(0.0));

/// ```css
/// min-width: auto;
/// ```
pub const AUTO: Property<Length> = Property(MIN_WIDTH, Length::Auto);

/// ```css
/// min-width: 1px;
/// ```
pub const PX: Property<Length> = Property(MIN_WIDTH, Length::Px(1.0));

/// ```css
/// min-width: 100%;
/// ```
pub const FULL: Property<Length> = Property(MIN_WIDTH, Length::Percent(100.0));

/// ```css
/// min-width: 100vw;
/// ```
pub const SCREEN: Property<Length> = Property(MIN_WIDTH, Length::Vw(100));

/// ```css
/// min-width: 100svw;
/// ```
pub const SVH: Property<Length> = Property(MIN_WIDTH, Length::Svw(100));

/// ```css
/// min-width: 100lvw;
/// ```
pub const LVH: Property<Length> = Property(MIN_WIDTH, Length::Lvw(100));

/// ```css
/// min-width: 100dvw;
/// ```
pub const DVH: Property<Length> = Property(MIN_WIDTH, Length::Dvw(100));

/// ```css
/// min-width: min-content;
/// ```
pub const MIN: Property<Length> = Property(MIN_WIDTH, Length::MinContent);

/// ```css
/// min-width: max-content;
/// ```
pub const MAX: Property<Length> = Property(MIN_WIDTH, Length::MaxContent);

/// ```css
/// min-width: fit-content;
/// ```
pub const FIT: Property<Length> = Property(MIN_WIDTH, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// min-width: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// min-width: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(x * 0.25))
}

/// ```css
/// min-width: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(f32::from(x)))
}

/// ```css
/// min-width: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Rem(x))
}

/// ```css
/// min-width: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Em(f32::from(x)))
}

/// ```css
/// min-width: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Em(x))
}

/// ```css
/// min-width: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// min-width: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Px(x))
}

/// ```css
/// min-width: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Percent(f32::from(x)))
}

/// ```css
/// min-width: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Percent(x))
}

/// ```css
/// min-width: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Vw(x))
}

/// ```css
/// min-width: {x}svw;
/// ```
pub fn svw(x: u16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Svw(x))
}

/// ```css
/// min-width: {x}lvw;
/// ```
pub fn lvw(x: u16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Lvw(x))
}

/// ```css
/// min-width: {x}dvw;
/// ```
pub fn dvw(x: u16) -> Property<Length> {
    Property(MIN_WIDTH, Length::Dvw(x))
}

/// ```css
/// min-width: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Mm(x))
}

/// ```css
/// min-width: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(MIN_WIDTH, Length::Cm(x))
}
