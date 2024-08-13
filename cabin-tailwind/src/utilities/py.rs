//! Set the padding area on the top and bottom of an element (`padding-top`, `padding-bottom`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-top>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/padding-bottom>

use crate::{Length, PropertyTwice};

const PADDING_TOP: &str = "padding-top";
const PADDING_BOTTOM: &str = "padding-bottom";

/// ```css
/// padding-bottom: 0; padding-bottom: 0;
/// ```
pub const ZERO: PropertyTwice<Length> = PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(0.0));

/// ```css
/// padding-top: auto; padding-bottom: auto;
/// ```
pub const AUTO: PropertyTwice<Length> = PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Auto);

/// ```css
/// padding-top: 1px; padding-bottom: 1px;
/// ```
pub const PX: PropertyTwice<Length> = PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding-top: {x * 0.25}rem; padding-bottom: {x * 0.25}rem
/// ```
pub fn unit(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        PADDING_TOP,
        PADDING_BOTTOM,
        Length::Rem(f32::from(x) * 0.25),
    )
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// padding-top: {x * 0.25}rem; padding-bottom: {x * 0.25}rem
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Rem(x * 0.25))
}

/// ```css
/// padding-top: {x}rem; padding-bottom: {x}rem;
/// ```
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Rem(f32::from(x)))
}

/// ```css
/// padding-top: {x}rem; padding-bottom: {x}rem;
/// ```
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Rem(x))
}

/// ```css
/// padding-top: {x}em; padding-bottom: {x}em;
/// ```
pub fn em(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Em(f32::from(x)))
}

/// ```css
/// padding-top: {x}em; padding-bottom: {x}em;
/// ```
pub fn emf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Em(x))
}

/// ```css
/// padding-top: {x}px; padding-bottom: {x}px;
/// ```
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(f32::from(x)))
}

/// ```css
/// padding-top: {x}px; padding-bottom: {x}px;
/// ```
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Px(x))
}

/// ```css
/// padding-top: {x}%; padding-bottomeft: {x}%;
/// ```
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Percent(f32::from(x)))
}

/// ```css
/// padding-top: {x}%; padding-bottomeft: {x}%;
/// ```
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Percent(x))
}

/// ```css
/// padding-top: {x}vh; padding-bottom: {x}vh;
/// ```
pub fn vh(x: u16) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Vh(x))
}

/// ```css
/// padding-top: {x}mm; padding-bottom: {x}mm;
/// ```
pub fn mm(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Mm(x))
}

/// ```css
/// padding-top: {x}cm padding-bottom: {x}cm;
/// ```
pub fn cm(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(PADDING_TOP, PADDING_BOTTOM, Length::Cm(x))
}
