//! Set the logical inline start position of a positioned element (`inset-inline-start`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/inset-inline-start>

use crate::tailwind::{Length, Property};

const INSET_INLINE_START: &str = "inset-inline-start";

/// ```css
/// inset-inline-start: 0;
/// ```
pub const ZERO: Property<Length> = Property(INSET_INLINE_START, Length::Px(0.0));

/// ```css
/// inset-inline-start: auto;
/// ```
pub const AUTO: Property<Length> = Property(INSET_INLINE_START, Length::Auto);

/// ```css
/// inset-inline-start: 1px;
/// ```
pub const PX: Property<Length> = Property(INSET_INLINE_START, Length::Px(1.0));

/// ```css
/// inset-inline-start: 100%;
/// ```
pub const FULL: Property<Length> = Property(INSET_INLINE_START, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// inset-inline-start: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// inset-inline-start: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Rem(x * 0.25))
}

/// ```css
/// inset-inline-start: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Rem(f32::from(x)))
}

/// ```css
/// inset-inline-start: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Rem(x))
}

/// ```css
/// inset-inline-start: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Em(f32::from(x)))
}

/// ```css
/// inset-inline-start: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Em(x))
}

/// ```css
/// inset-inline-start: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Px(f32::from(x)))
}

/// ```css
/// inset-inline-start: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Px(x))
}

/// ```css
/// inset-inline-start: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Percent(f32::from(x)))
}

/// ```css
/// inset-inline-start: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_START, Length::Percent(x))
}
