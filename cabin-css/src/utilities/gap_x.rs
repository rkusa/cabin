//! Set the gaps (gutters) between columns (`column-gap`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/column-gap>

use crate::{Length, Property};

const COLUMN_GAP: &str = "column-gap";

/// `column-gap: 0;`
pub const ZERO: Property<Length> = Property(COLUMN_GAP, Length::Px(0.0));

/// `column-gap: 1px;`
pub const PX: Property<Length> = Property(COLUMN_GAP, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `column-gap: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `column-gap: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(x * 0.25))
}

/// `column-gap: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(f32::from(x)))
}

/// `column-gap: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(COLUMN_GAP, Length::Rem(x))
}

/// `column-gap: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(COLUMN_GAP, Length::Px(f32::from(x)))
}

/// `column-gap: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(COLUMN_GAP, Length::Px(x))
}
