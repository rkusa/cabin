//! Set the padding area on the left and right of an element (`padding-left`, `padding-right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-left>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-right>

use crate::{Length, PropertyTwice};

const PADDING_LEFT: &str = "padding-left";
const PADDING_RIGHT: &str = "padding-right";

/// ```css
/// padding-right: 0; padding-right: 0;
/// ```
pub const ZERO: PropertyTwice<Length> = PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Px(0.0));

/// ```css
/// padding-left: auto; padding-right: auto;
/// ```
pub const AUTO: PropertyTwice<Length> = PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Auto);

/// ```css
/// padding-left: 1px; padding-right: 1px;
/// ```
pub const PX: PropertyTwice<Length> = PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding-left: {x * 0.25}rem; padding-right: {x * 0.25}rem
/// ```
pub fn unit(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(
        PADDING_LEFT,
        PADDING_RIGHT,
        Length::Rem(f32::from(x) * 0.25),
    )
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding-left: {x * 0.25}rem; padding-right: {x * 0.25}rem
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// padding-left: {x}rem; padding-right: {x}rem;
/// ```
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// padding-left: {x}rem; padding-right: {x}rem;
/// ```
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Rem(x))
}

/// ```css
/// padding-left: {x}px; padding-right: {x}px;
/// ```
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// padding-left: {x}px; padding-right: {x}px;
/// ```
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Px(x))
}

/// ```css
/// padding-left: {x}%; padding-righteft: {x}%;
/// ```
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Percent(f32::from(x)))
}

/// ```css
/// padding-left: {x}%; padding-righteft: {x}%;
/// ```
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Percent(x))
}

/// ```css
/// padding-left: {x}vw; padding-right: {x}vw;
/// ```
pub fn vw(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_LEFT, PADDING_RIGHT, Length::Vw(x))
}
