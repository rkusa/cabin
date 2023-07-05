//! Control the size of implicitly created grid columns (`grid-auto-columns`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-columns>

use crate::Property;

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
