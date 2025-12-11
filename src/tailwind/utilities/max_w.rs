//! Set the element's maximum width (`max-width`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>

use crate::tailwind::{Length, Property};

const MAX_WIDTH: &str = "max-width";

/// ```css
/// max-width: 0;
/// ```
pub const ZERO: Property<Length> = Property(MAX_WIDTH, Length::Px(0.0));

/// ```css
/// max-width: auto;
/// ```
pub const AUTO: Property<Length> = Property(MAX_WIDTH, Length::Auto);

/// ```css
/// max-width: 1px;
/// ```
pub const PX: Property<Length> = Property(MAX_WIDTH, Length::Px(1.0));

/// ```css
/// max-width: 100%;
/// ```
pub const FULL: Property<Length> = Property(MAX_WIDTH, Length::Percent(100.0));

/// ```css
/// max-width: 100vw;
/// ```
pub const SCREEN: Property<Length> = Property(MAX_WIDTH, Length::Vw(100));

/// ```css
/// max-width: 100svw;
/// ```
pub const SVH: Property<Length> = Property(MAX_WIDTH, Length::Svw(100));

/// ```css
/// max-width: 100lvw;
/// ```
pub const LVH: Property<Length> = Property(MAX_WIDTH, Length::Lvw(100));

/// ```css
/// max-width: 100dvw;
/// ```
pub const DVH: Property<Length> = Property(MAX_WIDTH, Length::Dvw(100));

/// ```css
/// max-width: min-content;
/// ```
pub const MIN: Property<Length> = Property(MAX_WIDTH, Length::MinContent);

/// ```css
/// max-width: max-content;
/// ```
pub const MAX: Property<Length> = Property(MAX_WIDTH, Length::MaxContent);

/// ```css
/// max-width: fit-content;
/// ```
pub const FIT: Property<Length> = Property(MAX_WIDTH, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// max-width: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// max-width: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(x * 0.25))
}

/// ```css
/// max-width: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(f32::from(x)))
}

/// ```css
/// max-width: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Rem(x))
}

/// ```css
/// max-width: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Em(f32::from(x)))
}

/// ```css
/// max-width: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Em(x))
}

/// ```css
/// max-width: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Px(f32::from(x)))
}

/// ```css
/// max-width: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Px(x))
}

/// ```css
/// max-width: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Percent(f32::from(x)))
}

/// ```css
/// max-width: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Percent(x))
}

/// ```css
/// max-width: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Vw(x))
}

/// ```css
/// max-width: {x}svw;
/// ```
pub fn svw(x: u16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Svw(x))
}

/// ```css
/// max-width: {x}lvw;
/// ```
pub fn lvw(x: u16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Lvw(x))
}

/// ```css
/// max-width: {x}dvw;
/// ```
pub fn dvw(x: u16) -> Property<Length> {
    Property(MAX_WIDTH, Length::Dvw(x))
}

/// ```css
/// max-width: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Mm(x))
}

/// ```css
/// max-width: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(MAX_WIDTH, Length::Cm(x))
}
