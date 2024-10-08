//! Set the logical inline start margin of an element (`margin-inline-start`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-start>

use crate::{Length, Property};

const MARGIN_INLINE_START: &str = "margin-inline-start";

/// ```css
/// margin-inline-start: 0;
/// ```
pub const ZERO: Property<Length> = Property(MARGIN_INLINE_START, Length::Px(0.0));

/// ```css
/// margin-inline-start: auto;
/// ```
pub const AUTO: Property<Length> = Property(MARGIN_INLINE_START, Length::Auto);

/// ```css
/// margin-inline-start: 1px;
/// ```
pub const PX: Property<Length> = Property(MARGIN_INLINE_START, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin-inline-start: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin-inline-start: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Rem(x * 0.25))
}

/// ```css
/// margin-inline-start: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Rem(f32::from(x)))
}

/// ```css
/// margin-inline-start: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Rem(x))
}

/// ```css
/// margin-inline-start: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Em(f32::from(x)))
}

/// ```css
/// margin-inline-start: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Em(x))
}

/// ```css
/// margin-inline-start: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Px(f32::from(x)))
}

/// ```css
/// margin-inline-start: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Px(x))
}

/// ```css
/// margin-inline-start: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Percent(f32::from(x)))
}

/// ```css
/// margin-inline-start: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_START, Length::Percent(x))
}
