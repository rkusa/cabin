//! Set the margin area on the left of an element (`margin-left`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-left>

use crate::{Length, Property};

const MARGIN_LEFT: &str = "margin-left";

/// `margin-left: 0;`
pub const ZERO: Property<Length> = Property(MARGIN_LEFT, Length::Px(0.0));

/// `margin-left: auto;`
pub const AUTO: Property<Length> = Property(MARGIN_LEFT, Length::Auto);

/// `margin-left: 1px;`
pub const PX: Property<Length> = Property(MARGIN_LEFT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `margin-left: {x * 0.25}rem;`
pub fn unit(x: i16) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `margin-left: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Rem(x * 0.25))
}

/// `margin-left: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Rem(f32::from(x)))
}

/// `margin-left: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Rem(x))
}

/// `margin-left: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Px(f32::from(x)))
}

/// `margin-left: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Px(x))
}

/// `margin-left: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Percent(f32::from(x)))
}

/// `margin-left: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Percent(x))
}

/// `margin-left: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Vw(x))
}

/// `margin-left: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(MARGIN_LEFT, Length::Vh(x))
}
