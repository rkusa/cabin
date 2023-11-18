//! Rounds the corners of an element's outer border edge (`border-radius`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>

pub mod b;
pub mod bl;
pub mod br;
pub mod l;
pub mod r;
pub mod t;
pub mod tl;
pub mod tr;

use crate::{Length, Property};
pub use b::DEFAULT as B;
pub use bl::DEFAULT as BL;
pub use br::DEFAULT as BR;
pub use l::DEFAULT as L;
pub use r::DEFAULT as R;
pub use t::DEFAULT as T;
pub use tl::DEFAULT as TL;
pub use tr::DEFAULT as TR;

const BORDER_RADIUS: &str = "border-radius";

include!(concat!(env!("OUT_DIR"), "/rounded.rs"));

/// ```css
/// border-radius: 0;
/// ```
pub const NONE: Property<Length> = Property(BORDER_RADIUS, Length::Px(0.0));

/// ```css
/// border-radius: 0.25rem;
/// ```
pub const DEFAULT: Property<Length> = Property(BORDER_RADIUS, Length::Rem(0.25));

/// ```css
/// border-radius: 9999px;
/// ```
pub const FULL: Property<Length> = Property(BORDER_RADIUS, Length::Px(9999.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// border-radius: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// border-radius: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Rem(x * 0.25))
}

/// ```css
/// border-radius: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Rem(f32::from(x)))
}

/// ```css
/// border-radius: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Rem(x))
}

/// ```css
/// border-radius: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Px(f32::from(x)))
}

/// ```css
/// border-radius: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Px(x))
}

/// ```css
/// border-radius: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Percent(f32::from(x)))
}

/// ```css
/// border-radius: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(BORDER_RADIUS, Length::Percent(x))
}
