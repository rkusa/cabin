//! Control direction of flex items (`flex-direction`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>

use crate::Property;

const FLEX_DIRECTION: &str = "flex-direction";

/// ```css
/// flex-direction: row;
/// ```
pub const ROW: Property = Property(FLEX_DIRECTION, "row");

/// ```css
/// flex-direction: row-reverse;
/// ```
pub const ROW_REVERSE: Property = Property(FLEX_DIRECTION, "row-reverse");

/// ```css
/// flex-direction: column;
/// ```
pub const COL: Property = Property(FLEX_DIRECTION, "column");

/// ```css
/// flex-direction: column-reverse;
/// ```
pub const COL_REVERSE: Property = Property(FLEX_DIRECTION, "column-reverse");
