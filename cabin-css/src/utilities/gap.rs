//! Set the gaps (gutters) between rows and columns (`gap`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/gap>

use crate::{Length, Property};

const GAP: &str = "gap";

/// `gap: 0;`
pub const ZERO: Property<Length> = Property(GAP, Length::Px(0.0));

/// `gap: 1px;`
pub const PX: Property<Length> = Property(GAP, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `gap: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(GAP, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `gap: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(GAP, Length::Rem(x * 0.25))
}

/// `gap: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(GAP, Length::Rem(f32::from(x)))
}

/// `gap: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(GAP, Length::Rem(x))
}

/// `gap: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(GAP, Length::Px(f32::from(x)))
}

/// `gap: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(GAP, Length::Px(x))
}
