//! Set the logical inline end position of a positioned element (`inset-inline-end`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/inset-inline-end>

use crate::tailwind::{Length, Property};

const INSET_INLINE_END: &str = "inset-inline-end";

/// ```css
/// inset-inline-end: 0;
/// ```
pub const ZERO: Property<Length> = Property(INSET_INLINE_END, Length::Px(0.0));

/// ```css
/// inset-inline-end: auto;
/// ```
pub const AUTO: Property<Length> = Property(INSET_INLINE_END, Length::Auto);

/// ```css
/// inset-inline-end: 1px;
/// ```
pub const PX: Property<Length> = Property(INSET_INLINE_END, Length::Px(1.0));

/// ```css
/// inset-inline-end: 100%;
/// ```
pub const FULL: Property<Length> = Property(INSET_INLINE_END, Length::Percent(100.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// inset-inline-end: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// inset-inline-end: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Rem(x * 0.25))
}

/// ```css
/// inset-inline-end: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Rem(f32::from(x)))
}

/// ```css
/// inset-inline-end: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Rem(x))
}

/// ```css
/// inset-inline-end: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Em(f32::from(x)))
}

/// ```css
/// inset-inline-end: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Em(x))
}

/// ```css
/// inset-inline-end: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Px(f32::from(x)))
}

/// ```css
/// inset-inline-end: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Px(x))
}

/// ```css
/// inset-inline-end: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Percent(f32::from(x)))
}

/// ```css
/// inset-inline-end: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(INSET_INLINE_END, Length::Percent(x))
}
