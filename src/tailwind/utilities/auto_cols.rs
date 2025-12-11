//! Control the size of implicitly created grid columns (`grid-auto-columns`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>

use crate::tailwind::{Length, Property};

const GRID_AUTO_COLUMNS: &str = "grid-auto-columns";

/// ```css
/// grid-auto-columns: auto;
/// ```
pub const AUTO: Property = Property(GRID_AUTO_COLUMNS, "auto");

/// ```css
/// grid-auto-columns: min-content;
/// ```
pub const MIN: Property = Property(GRID_AUTO_COLUMNS, "min-content");

/// ```css
/// grid-auto-columns: max-content;
/// ```
pub const MAX: Property = Property(GRID_AUTO_COLUMNS, "max-content");

/// ```css
/// grid-auto-columns: minmax(0, 1fr);
/// ```
pub const FR: Property = Property(GRID_AUTO_COLUMNS, "minmax(0, 1fr");

/// ```css
/// grid-auto-columns: {auto_cols};
/// ```
pub fn custom(auto_cols: &'static str) -> Property {
    Property(GRID_AUTO_COLUMNS, auto_cols)
}

/// ```css
/// grid-auto-columns: {x}px;
/// ```
pub fn px(x: i16) -> Property<Length> {
    Property(GRID_AUTO_COLUMNS, Length::Px(f32::from(x)))
}

/// ```css
/// grid-auto-columns: {x}px;
/// ```
pub fn pxf(x: f32) -> Property<Length> {
    Property(GRID_AUTO_COLUMNS, Length::Px(x))
}

/// ```css
/// grid-auto-columns: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property(GRID_AUTO_COLUMNS, Length::Percent(f32::from(x)))
}

/// ```css
/// grid-auto-columns: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property(GRID_AUTO_COLUMNS, Length::Percent(x))
}

/// ```css
/// grid-auto-columns: {x}vw;
/// ```
pub fn vw(x: u16) -> Property<Length> {
    Property(GRID_AUTO_COLUMNS, Length::Vw(x))
}
