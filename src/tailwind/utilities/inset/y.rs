//! Set vertical position of a positioned element (`top`/`bottom`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/top>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>

use crate::tailwind::{Length, PropertyTwice};

const TOP: &str = "top";
const BOTTOM: &str = "bottom";

/// ```css
/// top: 0;
/// bottom: 0;
/// ```
pub const ZERO: PropertyTwice<Length> = PropertyTwice(TOP, BOTTOM, Length::Px(0.0));

/// ```css
/// top: auto;
/// bottom: auto;
/// ```
pub const AUTO: PropertyTwice<Length> = PropertyTwice(TOP, BOTTOM, Length::Auto);

/// ```css
/// top: 1px;
/// bottom: 1px;
/// ```
pub const PX: PropertyTwice<Length> = PropertyTwice(TOP, BOTTOM, Length::Px(1.0));

/// ```css
/// top: 100%;
/// bottom: 100%;
/// ```
pub const FULL: PropertyTwice<Length> = PropertyTwice(TOP, BOTTOM, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// top: {x * 0.25}rem;
/// bottom: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// top: {x * 0.25}rem;
/// bottom: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Rem(x * 0.25))
}

/// ```css
/// top: {x}rem;
/// bottom: {x}rem;
/// ```
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Rem(f32::from(x)))
}

/// ```css
/// top: {x}rem;
/// bottom: {x}rem;
/// ```
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Rem(x))
}

/// ```css
/// top: {x}px;
/// bottom: {x}px;
/// ```
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Px(f32::from(x)))
}

/// ```css
/// top: {x}px;
/// bottom: {x}px;
/// ```
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Px(x))
}

/// ```css
/// top: {x}%;
/// bottom: {x}%;
/// ```
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Percent(f32::from(x)))
}

/// ```css
/// top: {x}%;
/// bottom: {x}%;
/// ```
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Percent(x))
}

/// ```css
/// top: {x}vh;
/// bottom: {x}vh;
/// ```
pub fn vh(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(TOP, BOTTOM, Length::Vw(x))
}
