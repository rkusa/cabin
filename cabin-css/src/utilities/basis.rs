//! Set the initial main size of a flex item (`flex-basis`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis>

use crate::{Length, Property};

const FLEX_BASIS: &str = "flex-basis";

/// `flex-basis: 0;`
pub const ZERO: Property<Length> = Property(FLEX_BASIS, Length::Px(0.0));

/// `flex-basis: auto;`
pub const AUTO: Property<Length> = Property(FLEX_BASIS, Length::Auto);

/// `flex-basis: 1px;`
pub const PX: Property<Length> = Property(FLEX_BASIS, Length::Px(1.0));

/// `flex-basis: 100%;`
pub const FULL: Property<Length> = Property(FLEX_BASIS, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default): `flex-basis: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(FLEX_BASIS, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `flex-basis: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(FLEX_BASIS, Length::Rem(x * 0.25))
}

/// `flex-basis: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(FLEX_BASIS, Length::Rem(f32::from(x)))
}

/// `flex-basis: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(FLEX_BASIS, Length::Rem(x))
}

/// `flex-basis: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(FLEX_BASIS, Length::Px(f32::from(x)))
}

/// `flex-basis: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(FLEX_BASIS, Length::Px(x))
}

/// `flex-basis: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(FLEX_BASIS, Length::Percent(f32::from(x)))
}

/// `flex-basis: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(FLEX_BASIS, Length::Percent(x))
}
