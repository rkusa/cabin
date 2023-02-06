//! Set the padding area on the top and bottom of an element (`padding-top`, `padding-bottom`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-top>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-bottom>

use crate::{Length, PropertyTwice};

const PADDING_TOP: &str = "padding-top";
const PADDING_BOTTOM: &str = "padding-bottom";

/// `padding-bottom: 0; padding-bottom: 0;`
pub const ZERO: PropertyTwice<Length> = PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(0.0));

/// `padding-top: auto; padding-bottom: auto;`
pub const AUTO: PropertyTwice<Length> = PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Auto);

/// `padding-top: 1px; padding-bottom: 1px;`
pub const PX: PropertyTwice<Length> = PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `padding-top: {x * 0.25}rem; padding-bottom: {x * 0.25}rem`
pub fn unit(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `padding-top: {x * 0.25}rem; padding-bottom: {x * 0.25}rem`
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Rem(x * 0.25))
}

/// `padding-top: {x}rem; padding-bottom: {x}rem;`
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Rem(f32::from(x)))
}

/// `padding-top: {x}rem; padding-bottom: {x}rem;`
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Rem(x))
}

/// `padding-top: {x}px; padding-bottom: {x}px;`
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(f32::from(x)))
}

/// `padding-top: {x}px; padding-bottom: {x}px;`
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(x))
}

/// `padding-top: {x}%; padding-bottomeft: {x}%;`
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Percent(f32::from(x)))
}

/// `padding-top: {x}%; padding-bottomeft: {x}%;`
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Percent(x))
}

/// `padding-top: {x}vw; padding-bottom: {x}vw;`
pub fn vw(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Vw(x))
}

/// `padding-top: {x}vh; padding-bottom: {x}vh;`
pub fn vh(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Vh(x))
}
