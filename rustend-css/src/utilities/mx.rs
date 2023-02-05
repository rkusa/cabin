//! Set the margin area on the left and right of an element (`margin-left`, `margin-right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-left>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-right>

use crate::{Length, PropertyTwice};

const MARGIN_LEFT: &str = "margin-left";
const MARGIN_RIGHT: &str = "margin-right";

/// `margin-right: 0; margin-right: 0;`
pub const ZERO: PropertyTwice<Length> = PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(0.0));

/// `margin-left: auto; margin-right: auto;`
pub const AUTO: PropertyTwice<Length> = PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Auto);

/// `margin-left: 1px; margin-right: 1px;`
pub const PX: PropertyTwice<Length> = PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `margin-left: {x * 0.25}rem; margin-right: {x * 0.25}rem`
pub fn unit(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `margin-left: {x * 0.25}rem; margin-right: {x * 0.25}rem`
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(x * 0.25))
}

/// `margin-left: {x}rem; margin-right: {x}rem;`
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(f32::from(x)))
}

/// `margin-left: {x}rem; margin-right: {x}rem;`
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(x))
}

/// `margin-left: {x}px; margin-right: {x}px;`
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(f32::from(x)))
}

/// `margin-left: {x}px; margin-right: {x}px;`
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(x))
}

/// `margin-left: {x}%; margin-righteft: {x}%;`
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Percent(f32::from(x)))
}

/// `margin-left: {x}%; margin-righteft: {x}%;`
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Percent(x))
}

/// `margin-left: {x}vw; margin-right: {x}vw;`
pub fn vw(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Vw(x))
}

/// `margin-left: {x}vh; margin-right: {x}vh;`
pub fn vh(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Vh(x))
}
