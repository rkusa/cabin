//! Set the margin area on the top of an element (`margin-top`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-top>

use crate::{Length, Property};

const MARGIN_TOP: &str = "margin-top";

/// `margin-top: 0;`
pub const ZERO: Property<Length> = Property(MARGIN_TOP, Length::Px(0.0));

/// `margin-top: auto;`
pub const AUTO: Property<Length> = Property(MARGIN_TOP, Length::Auto);

/// `margin-top: 1px;`
pub const PX: Property<Length> = Property(MARGIN_TOP, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `margin-top: {x * 0.25}rem;`
pub fn unit(x: i16) -> Property<Length> {
    Property(MARGIN_TOP, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `margin-top: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(MARGIN_TOP, Length::Rem(x * 0.25))
}

/// `margin-top: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(MARGIN_TOP, Length::Rem(f32::from(x)))
}

/// `margin-top: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(MARGIN_TOP, Length::Rem(x))
}

/// `margin-top: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(MARGIN_TOP, Length::Px(f32::from(x)))
}

/// `margin-top: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(MARGIN_TOP, Length::Px(x))
}

/// `margin-top: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(MARGIN_TOP, Length::Percent(f32::from(x)))
}

/// `margin-top: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(MARGIN_TOP, Length::Percent(x))
}

/// `margin-top: {x}vw;`
pub fn vw(x: u16) -> Property<Length> {
    Property(MARGIN_TOP, Length::Vw(x))
}

/// `margin-top: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(MARGIN_TOP, Length::Vh(x))
}
