//! Set the gaps (gutters) between rows (`row-gap`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/row-gap>

use crate::{Length, Property};

const ROW_GAP: &str = "row-gap";

/// `row-gap: 0;`
pub const ZERO: Property<Length> = Property(ROW_GAP, Length::Px(0.0));

/// `row-gap: 1px;`
pub const PX: Property<Length> = Property(ROW_GAP, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `row-gap: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(ROW_GAP, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `row-gap: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(ROW_GAP, Length::Rem(x * 0.25))
}

/// `row-gap: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(ROW_GAP, Length::Rem(f32::from(x)))
}

/// `row-gap: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(ROW_GAP, Length::Rem(x))
}

/// `row-gap: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(ROW_GAP, Length::Px(f32::from(x)))
}

/// `row-gap: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(ROW_GAP, Length::Px(x))
}
