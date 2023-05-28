//! Rounds the corners of an element's left outer border edge (`border-top-left-radius` and
//! `border-bottom-left-radius`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-top-left-radius>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-bottom-left-radius>

use crate::{Length, PropertyTwice};

const BORDER_TOP_LEFT_RADIUS: &str = "border-top-left-radius";
const BORDER_BOTTOM_LEFT_RADIUS: &str = "border-bottom-left-radius";

include!(concat!(env!("OUT_DIR"), "/rounded-l.rs"));

/// `border-top-left-radius: 0; border-bottom-left-radius: 0;`
pub const NONE: PropertyTwice<Length> = PropertyTwice(
    BORDER_TOP_LEFT_RADIUS,
    BORDER_BOTTOM_LEFT_RADIUS,
    Length::Px(0.0),
);

/// `border-top-left-radius: 0.25rem; border-bottom-left-radius: 0.25rem;`
pub const DEFAULT: PropertyTwice<Length> = PropertyTwice(
    BORDER_TOP_LEFT_RADIUS,
    BORDER_BOTTOM_LEFT_RADIUS,
    Length::Rem(0.25),
);

/// `border-top-left-radius: 9999px; border-bottom-left-radius: 9999px;`
pub const FULL: PropertyTwice<Length> = PropertyTwice(
    BORDER_TOP_LEFT_RADIUS,
    BORDER_BOTTOM_LEFT_RADIUS,
    Length::Px(9999.0),
);

/// Multiple of `0.25rem` (`4px` by default):
/// ```
/// border-top-left-radius: {x * 0.25}rem;
/// border-bottom-left-radius: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Rem(f32::from(x) * 0.25),
    )
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```
/// border-top-left-radius: {x * 0.25}rem;
/// border-bottom-left-radius: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Rem(x * 0.25),
    )
}

/// `border-top-left-radius: {x}rem; border-bottom-left-radius: {x}rem;`
pub fn rem(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Rem(f32::from(x)),
    )
}

/// `border-top-left-radius: {x}rem; border-bottom-left-radius: {x}rem;`
pub fn remf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Rem(x),
    )
}

/// `border-top-left-radius: {x}px; border-bottom-left-radius: {x}px;`
pub fn px(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Px(f32::from(x)),
    )
}

/// `border-top-left-radius: {x}px; border-bottom-left-radius: {x}px;`
pub fn pxf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Px(x),
    )
}

/// `border-top-left-radius: {x}%; border-bottom-left-radius: {x}%;`
pub fn percent(x: i16) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Percent(f32::from(x)),
    )
}

/// `border-top-left-radius: {x}%; border-bottom-left-radius: {x}%;`
pub fn percentf(x: f32) -> PropertyTwice<Length> {
    PropertyTwice(
        BORDER_TOP_LEFT_RADIUS,
        BORDER_BOTTOM_LEFT_RADIUS,
        Length::Percent(x),
    )
}
