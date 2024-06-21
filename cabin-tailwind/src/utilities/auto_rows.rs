//! Control the size of implicitly created grid rows (`grid-auto-rows`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>

use crate::{Length, Property};

const GRID_AUTO_ROWS: &str = "grid-auto-rows";

/// ```css
/// grid-auto-rows: auto;
/// ```
pub const AUTO: Property = Property(GRID_AUTO_ROWS, "auto");

/// ```css
/// grid-auto-rows: min-content;
/// ```
pub const MIN: Property = Property(GRID_AUTO_ROWS, "min-content");

/// ```css
/// grid-auto-rows: max-content;
/// ```
pub const MAX: Property = Property(GRID_AUTO_ROWS, "max-content");

/// ```css
/// grid-auto-rows: minmax(0, 1fr);
/// ```
pub const FR: Property = Property(GRID_AUTO_ROWS, "minmax(0, 1fr");

/// ```css
/// grid-auto-rows: {auto_rows};
/// ```
pub fn custom(auto_rows: &'static str) -> Property {
    Property(GRID_AUTO_ROWS, auto_rows)
}

/// ```css
/// grid-auto-rows: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(GRID_AUTO_ROWS, Length::Px(f32::from(x)))
}

/// ```css
/// grid-auto-rows: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(GRID_AUTO_ROWS, Length::Px(x))
}

/// ```css
/// grid-auto-rows: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(GRID_AUTO_ROWS, Length::Percent(f32::from(x)))
}

/// ```css
/// grid-auto-rows: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(GRID_AUTO_ROWS, Length::Percent(x))
}

/// ```css
/// grid-auto-rows: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(GRID_AUTO_ROWS, Length::Vw(x))
}
