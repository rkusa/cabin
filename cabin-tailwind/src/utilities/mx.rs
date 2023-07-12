//! Set the margin area on the left and right of an element (`margin-left`, `margin-right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-left>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-right>

use crate::{Length, PropertyTwice};

const MARGIN_LEFT: &str = "margin-left";
const MARGIN_RIGHT: &str = "margin-right";

/// ```css
/// margin-right: 0; margin-right: 0;
/// ```
pub const ZERO: PropertyTwice<Length> = PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(0.0));

/// ```css
/// margin-left: auto; margin-right: auto;
/// ```
pub const AUTO: PropertyTwice<Length> = PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Auto);

/// ```css
/// margin-left: 1px; margin-right: 1px;
/// ```
pub const PX: PropertyTwice<Length> = PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin-left: {x * 0.25}rem; margin-right: {x * 0.25}rem
/// ```
pub fn unit(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin-left: {x * 0.25}rem; margin-right: {x * 0.25}rem
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// margin-left: {x}rem; margin-right: {x}rem;
/// ```
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// margin-left: {x}rem; margin-right: {x}rem;
/// ```
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Rem(x))
}

/// ```css
/// margin-left: {x}px; margin-right: {x}px;
/// ```
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// margin-left: {x}px; margin-right: {x}px;
/// ```
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Px(x))
}

/// ```css
/// margin-left: {x}%; margin-righteft: {x}%;
/// ```
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Percent(f32::from(x)))
}

/// ```css
/// margin-left: {x}%; margin-righteft: {x}%;
/// ```
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Percent(x))
}

/// ```css
/// margin-left: {x}vw; margin-right: {x}vw;
/// ```
pub fn vw(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(MARGIN_LEFT, MARGIN_RIGHT, Length::Vw(x))
}
