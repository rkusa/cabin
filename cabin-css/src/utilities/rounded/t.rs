//! Rounds the corners of an element's top outer border edge (`border-top-right-radius` and
//! `border-top-left-radius`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-right-radius>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>

use crate::{Length, PropertyTwice};

const BORDER_TOP_RIGHT_RADIUS: &str = "border-top-right-radius";
const BORDER_TOP_LEFT_RADIUS: &str = "border-top-left-radius";

include!(concat!(env!("OUT_DIR"), "/rounded-t.rs"));

/// `border-top-right-radius: 0; border-top-left-radius: 0;`
pub const NONE: PropertyTwice<Length> = PropertyTwice(
    BORDER_TOP_RIGHT_RADIUS,
    BORDER_TOP_LEFT_RADIUS,
    Length::Px(0.0),
);

/// `border-top-right-radius: 0.25rem; border-top-left-radius: 0.25rem;`
pub const DEFAULT: PropertyTwice<Length> = PropertyTwice(
    BORDER_TOP_RIGHT_RADIUS,
    BORDER_TOP_LEFT_RADIUS,
    Length::Rem(0.25),
);

/// `border-top-right-radius: 9999px; border-top-left-radius: 9999px;`
pub const FULL: PropertyTwice<Length> = PropertyTwice(
    BORDER_TOP_RIGHT_RADIUS,
    BORDER_TOP_LEFT_RADIUS,
    Length::Px(9999.0),
);

/// Multiple of `0.25rem` (`4px` by default):
/// ```
/// border-top-right-radius: {x * 0.25}rem;
/// border-top-left-radius: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Rem(f32::from(x) * 0.25),
    )
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```
/// border-top-right-radius: {x * 0.25}rem;
/// border-top-left-radius: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Rem(x * 0.25),
    )
}

/// `border-top-right-radius: {x}rem; border-top-left-radius: {x}rem;`
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Rem(f32::from(x)),
    )
}

/// `border-top-right-radius: {x}rem; border-top-left-radius: {x}rem;`
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Rem(x),
    )
}

/// `border-top-right-radius: {x}px; border-top-left-radius: {x}px;`
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Px(f32::from(x)),
    )
}

/// `border-top-right-radius: {x}px; border-top-left-radius: {x}px;`
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Px(x),
    )
}

/// `border-top-right-radius: {x}%; border-top-left-radius: {x}%;`
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Percent(f32::from(x)),
    )
}

/// `border-top-right-radius: {x}%; border-top-left-radius: {x}%;`
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_RIGHT_RADIUS,
        BORDER_TOP_LEFT_RADIUS,
        Length::Percent(x),
    )
}
