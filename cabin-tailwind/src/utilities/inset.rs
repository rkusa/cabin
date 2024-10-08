//! Set horizontal and vertical position of a positioned element (`inset`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/inset>

pub mod x;
pub mod y;

pub use x::unit as x;
pub use y::unit as y;

use crate::{Length, Property};

const INSET: &str = "inset";

/// ```css
/// inset: 0;
/// ```
pub const ZERO: Property<Length> = Property(INSET, Length::Px(0.0));

/// ```css
/// inset: auto;
/// ```
pub const AUTO: Property<Length> = Property(INSET, Length::Auto);

/// ```css
/// inset: 1px;
/// ```
pub const PX: Property<Length> = Property(INSET, Length::Px(1.0));

/// ```css
/// inset: 100%;
/// ```
pub const FULL: Property<Length> = Property(INSET, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// inset: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(INSET, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// inset: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(INSET, Length::Rem(x * 0.25))
}

/// ```css
/// inset: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(INSET, Length::Rem(f32::from(x)))
}

/// ```css
/// inset: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(INSET, Length::Rem(x))
}

/// ```css
/// inset: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(INSET, Length::Em(f32::from(x)))
}

/// ```css
/// inset: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(INSET, Length::Em(x))
}

/// ```css
/// inset: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(INSET, Length::Px(f32::from(x)))
}

/// ```css
/// inset: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(INSET, Length::Px(x))
}

/// ```css
/// inset: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(INSET, Length::Percent(f32::from(x)))
}

/// ```css
/// inset: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(INSET, Length::Percent(x))
}

/// ```css
/// inset: {x}mm;
/// ```
pub fn mm(x: f32) -> Property<Length> {
    Property(INSET, Length::Mm(x))
}

/// ```css
/// inset: {x}cm;
/// ```
pub fn cm(x: f32) -> Property<Length> {
    Property(INSET, Length::Cm(x))
}
