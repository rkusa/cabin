//! Set the margin area on the top and bottom of an element (`margin-top`, `margin-bottom`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-top>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-bottom>

use crate::{Length, PropertyTwice};

const MARGIN_TOP: &str = "margin-top";
const MARGIN_BOTTOM: &str = "margin-bottom";

/// `margin-bottom: 0; margin-bottom: 0;`
pub const ZERO: PropertyTwice<Length> = PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Px(0.0));

/// `margin-top: auto; margin-bottom: auto;`
pub const AUTO: PropertyTwice<Length> = PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Auto);

/// `margin-top: 1px; margin-bottom: 1px;`
pub const PX: PropertyTwice<Length> = PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default): `margin-top: {x * 0.25}rem; margin-bottom: {x * 0.25}rem`
pub fn unit(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `margin-top: {x * 0.25}rem; margin-bottom: {x * 0.25}rem`
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Rem(x * 0.25))
}

/// `margin-top: {x}rem; margin-bottom: {x}rem;`
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Rem(f32::from(x)))
}

/// `margin-top: {x}rem; margin-bottom: {x}rem;`
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Rem(x))
}

/// `margin-top: {x}px; margin-bottom: {x}px;`
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Px(f32::from(x)))
}

/// `margin-top: {x}px; margin-bottom: {x}px;`
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Px(x))
}

/// `margin-top: {x}%; margin-bottomeft: {x}%;`
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Percent(f32::from(x)))
}

/// `margin-top: {x}%; margin-bottomeft: {x}%;`
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Percent(x))
}

/// `margin-top: {x}vh; margin-bottom: {x}vh;`
pub fn vh(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_TOP, MARGIN_BOTTOM, Length::Vh(x))
}
