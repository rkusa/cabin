//! Rounds the corners of an element's right outer border edge (`border-top-right-radius` and
//! `border-bottom-right-radius`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-right-radius>

use crate::{Length, PropertyTwice};

const BORDER_TOP_RIGHT_RADIUS: &str = "border-top-right-radius";
const BORDER_BOTTOM_RIGHT_RADIUS: &str = "border-bottom-right-radius";

include!(concat!(env!("OUT_DIR"), "/rounded-r.rs"));

/// ```css
/// border-top-right-radius: 0; border-bottom-right-radius: 0;
/// ```
pub const NONE: PropertyTwice<Length, 1> = PropertyTwice(
    BORDER_TOP_RIGHT_RADIUS,
    BORDER_BOTTOM_RIGHT_RADIUS,
    Length::Px(0.0),
);

/// ```css
/// border-top-right-radius: 0.25rem; border-bottom-right-radius: 0.25rem;
/// ```
pub const DEFAULT: PropertyTwice<Length, 1> = PropertyTwice(
    BORDER_TOP_RIGHT_RADIUS,
    BORDER_BOTTOM_RIGHT_RADIUS,
    Length::Rem(0.25),
);

/// ```css
/// border-top-right-radius: 9999px; border-bottom-right-radius: 9999px;
/// ```
pub const FULL: PropertyTwice<Length, 1> = PropertyTwice(
    BORDER_TOP_RIGHT_RADIUS,
    BORDER_BOTTOM_RIGHT_RADIUS,
    Length::Px(9999.0),
);

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// border-top-right-radius: {x * 0.25}rem;
/// border-bottom-right-radius: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Rem(f32::from(x) * 0.25),
    )
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// border-top-right-radius: {x * 0.25}rem;
/// border-bottom-right-radius: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Rem(x * 0.25),
    )
}

/// ```css
/// border-top-right-radius: {x}rem; border-bottom-right-radius: {x}rem;
/// ```
pub fn rem(x: i16) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Rem(f32::from(x)),
    )
}

/// ```css
/// border-top-right-radius: {x}rem; border-bottom-right-radius: {x}rem;
/// ```
pub fn remf(x: f32) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Rem(x),
    )
}

/// ```css
/// border-top-right-radius: {x}px; border-bottom-right-radius: {x}px;
/// ```
pub fn px(x: i16) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Px(f32::from(x)),
    )
}

/// ```css
/// border-top-right-radius: {x}px; border-bottom-right-radius: {x}px;
/// ```
pub fn pxf(x: f32) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Px(x),
    )
}

/// ```css
/// border-top-right-radius: {x}%; border-bottom-right-radius: {x}%;
/// ```
pub fn percent(x: i16) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Percent(f32::from(x)),
    )
}

/// ```css
/// border-top-right-radius: {x}%; border-bottom-right-radius: {x}%;
/// ```
pub fn percentf(x: f32) -> PropertyTwice<Length, 1> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_BOTTOM_RIGHT_RADIUS,
        Length::Percent(x),
    )
}
