//! Set the gaps (gutters) between columns (`column-gap`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/column-gap>

use crate::{Length, Property};

const COLUMN_GAP: &str = "column-gap";

/// ```css
/// column-gap: 0;
/// ```
pub const ZERO: Property<Length> = Property(COLUMN_GAP, Length::Px(0.0));

/// ```css
/// column-gap: 1px;
/// ```
pub const PX: Property<Length> = Property(COLUMN_GAP, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// column-gap: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// column-gap: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(x * 0.25))
}

/// ```css
/// column-gap: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(f32::from(x)))
}

/// ```css
/// column-gap: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(x))
}

/// ```css
/// column-gap: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(COLUMN_GAP, Length::Em(f32::from(x)))
}

/// ```css
/// column-gap: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(COLUMN_GAP, Length::Em(x))
}

/// ```css
/// column-gap: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(COLUMN_GAP, Length::Px(f32::from(x)))
}

/// ```css
/// column-gap: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(COLUMN_GAP, Length::Px(x))
}
