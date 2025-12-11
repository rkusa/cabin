//! Set how elements in a grid are auto-placed (`grid-auto-flow`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/grid-auto-flow>

use crate::tailwind::Property;

const GRID_AUTO_FLOW: &str = "grid-auto-flow";

/// ```css
/// grid-auto-flow: row;
/// ```
pub const ROW: Property = Property(GRID_AUTO_FLOW, "row");

/// ```css
/// grid-auto-flow: column;
/// ```
pub const COL: Property = Property(GRID_AUTO_FLOW, "column");

/// ```css
/// grid-auto-flow: dense;
/// ```
pub const DENSE: Property = Property(GRID_AUTO_FLOW, "dense");

/// ```css
/// grid-auto-flow: row dense;
/// ```
pub const ROW_DENSE: Property = Property(GRID_AUTO_FLOW, "row dense");

/// ```css
/// grid-auto-flow: column dense;
/// ```
pub const COL_DENSE: Property = Property(GRID_AUTO_FLOW, "column dense");
