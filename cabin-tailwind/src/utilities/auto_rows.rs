//! Control the size of implicitly created grid rows (`grid-auto-rows`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-rows>

use crate::Property;

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
