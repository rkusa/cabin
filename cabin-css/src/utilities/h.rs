//! Set the element's height (`height`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/height>

use crate::{Length, Property};

const HEIGHT: &str = "height";

/// `height: 0;`
pub const ZERO: Property<Length> = Property(HEIGHT, Length::Px(0.0));

/// `height: auto;`
pub const AUTO: Property<Length> = Property(HEIGHT, Length::Auto);

/// `height: 1px;`
pub const PX: Property<Length> = Property(HEIGHT, Length::Px(1.0));

/// `height: 100%;`
pub const FULL: Property<Length> = Property(HEIGHT, Length::Percent(100.0));

/// `height: 100vw;`
pub const SCREEN: Property<Length> = Property(HEIGHT, Length::Vw(100));

/// `height: min-content;`
pub const MIN: Property<Length> = Property(HEIGHT, Length::MinContent);

/// `height: max-content;`
pub const MAX: Property<Length> = Property(HEIGHT, Length::MaxContent);

/// `height: fit-content;`
pub const FIT: Property<Length> = Property(HEIGHT, Length::FitContent);

/// Multiple of `0.25rem` (`4px` by default): `height: {x * 0.25}rem;`
pub fn unit(x: u16) -> Property<Length> {
    Property(HEIGHT, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default): `height: {x * 0.25}rem;`
pub fn unitf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Rem(x * 0.25))
}

/// `height: {x}rem;`
pub fn rem(x: i16) -> Property<Length> {
    Property(HEIGHT, Length::Rem(f32::from(x)))
}

/// `height: {x}rem;`
pub fn remf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Rem(x))
}

/// `height: {x}px;`
pub fn px(x: i16) -> Property<Length> {
    Property(HEIGHT, Length::Px(f32::from(x)))
}

/// `height: {x}px;`
pub fn pxf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Px(x))
}

/// `height: {x}%;`
pub fn percent(x: i16) -> Property<Length> {
    Property(HEIGHT, Length::Percent(f32::from(x)))
}

/// `height: {x}%;`
pub fn percentf(x: f32) -> Property<Length> {
    Property(HEIGHT, Length::Percent(x))
}

/// `height: {x}vh;`
pub fn vh(x: u16) -> Property<Length> {
    Property(HEIGHT, Length::Vh(x))
}
