//! Set the logical inline start margin of an element (`margin-inline-end`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/margin-inline-end>

use crate::{Length, Property};

const MARGIN_INLINE_END: &str = "margin-inline-end";

/// ```css
/// margin-inline-end: 0;
/// ```
pub const ZERO: Property<Length> = Property(MARGIN_INLINE_END, Length::Px(0.0));

/// ```css
/// margin-inline-end: auto;
/// ```
pub const AUTO: Property<Length> = Property(MARGIN_INLINE_END, Length::Auto);

/// ```css
/// margin-inline-end: 1px;
/// ```
pub const PX: Property<Length> = Property(MARGIN_INLINE_END, Length::Px(1.0));

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin-inline-end: {x * 0.25}rem;
/// ```
pub fn unit(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Rem(f32::from(x) * 0.25))
}

/// Multiple of `0.25rem` (`4px` by default):
/// ```css
/// margin-inline-end: {x * 0.25}rem;
/// ```
pub fn unitf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Rem(x * 0.25))
}

/// ```css
/// margin-inline-end: {x}rem;
/// ```
pub fn rem(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Rem(f32::from(x)))
}

/// ```css
/// margin-inline-end: {x}rem;
/// ```
pub fn remf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Rem(x))
}

/// ```css
/// margin-inline-end: {x}em;
/// ```
pub fn em(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Em(f32::from(x)))
}

/// ```css
/// margin-inline-end: {x}em;
/// ```
pub fn emf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Em(x))
}

/// ```css
/// margin-inline-end: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Px(f32::from(x)))
}

/// ```css
/// margin-inline-end: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Px(x))
}

/// ```css
/// margin-inline-end: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Percent(f32::from(x)))
}

/// ```css
/// margin-inline-end: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(MARGIN_INLINE_END, Length::Percent(x))
}
