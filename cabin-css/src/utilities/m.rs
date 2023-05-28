//! Set the margin area on all four sides of an element (`margin`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>

use crate::{Length, Property};

const MARGIN: &str = "margin";

/// `margin: 0;`
pub const ZERO: Property<Length> = Property(MARGIN, Length::Px(0.0));

/// `margin: auto;`
pub const AUTO: Property<Length> = Property(MARGIN, Length::Auto);

/// `margin: 1px;`
pub const PX: Property<Length> = Property(MARGIN, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `margin: {x * 0.25}rem;`
pub fn unit(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `margin: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Rem(x * 0.25))
}

/// `margin: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Rem(f32::from(x)))
}

/// `margin: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Rem(x))
}

/// `margin: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Px(f32::from(x)))
}

/// `margin: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Px(x))
}

/// `margin: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(MARGIN, Length::Percent(f32::from(x)))
}

/// `margin: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(MARGIN, Length::Percent(x))
}

/// `margin: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(MARGIN, Length::Vw(x))
}

/// `margin: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(MARGIN, Length::Vh(x))
}
