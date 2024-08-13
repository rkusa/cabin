//! Set horizontal position of a positioned element (`left`/`right`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/left>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/right>

use crate::{Length, PropertyTwice};

const LEFT: &str = "left";
const RIGHT: &str = "right";

/// ```css
/// left: 0;
/// right: 0;
/// ```
pub const ZERO: PropertyTwice<Length> = PropertyTwice(LEFT, RIGHT, Length::Px(0.0));

/// ```css
/// left: auto;
/// right: auto;
/// ```
pub const AUTO: PropertyTwice<Length> = PropertyTwice(LEFT, RIGHT, Length::Auto);

/// ```css
/// left: 1px;
/// right: 1px;
/// ```
pub const PX: PropertyTwice<Length> = PropertyTwice(LEFT, RIGHT, Length::Px(1.0));

/// ```css
/// left: 100%;
/// right: 100%;
/// ```
pub const FULL: PropertyTwice<Length> = PropertyTwice(LEFT, RIGHT, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// left: {x * 0.25}rem;
/// right: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// left: {x * 0.25}rem;
/// right: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Rem(x * 0.25))
}

/// ```css
/// left: {x}rem;
/// right: {x}rem;
/// ```
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Rem(f32::from(x)))
}

/// ```css
/// left: {x}rem;
/// right: {x}rem;
/// ```
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Rem(x))
}

/// ```css
/// left: {x}px;
/// right: {x}px;
/// ```
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Px(f32::from(x)))
}

/// ```css
/// left: {x}px;
/// right: {x}px;
/// ```
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Px(x))
}

/// ```css
/// left: {x}%;
/// right: {x}%;
/// ```
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Percent(f32::from(x)))
}

/// ```css
/// left: {x}%;
/// right: {x}%;
/// ```
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Percent(x))
}

/// ```css
/// left: {x}vw;
/// right: {x}vw;
/// ```
pub fn vw(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(LEFT, RIGHT, Length::Vw(x))
}
