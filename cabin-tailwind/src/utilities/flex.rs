//! Control direction of flex items (`flex-direction`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>

use crate::Property;

const FLEX_DIRECTION: &str = "flex-direction";
const FLEX_SHRINK: &str = "flex-shrink";
const FLEX_GROW: &str = "flex-grow";
const FLEX_WRAP: &str = "flex-wrap";

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

/// ```css
/// flex-shrink: 1;
/// ```
pub const SHRINK: Property = Property(FLEX_SHRINK, "1");

/// ```css
/// flex-shrink: 0;
/// ```
pub const NO_SHRINK: Property = Property(FLEX_SHRINK, "0");

/// ```css
/// flex-grow: 1;
/// ```
pub const GROW: Property = Property(FLEX_GROW, "1");

/// ```css
/// flex-grow: 0;
/// ```
pub const NO_GROW: Property = Property(FLEX_GROW, "0");

/// ```css
/// flex-wrap: wrap;
/// ```
pub const WRAP: Property = Property(FLEX_WRAP, "wrap");

/// ```css
/// flex-wrap: wrap-reverse;
/// ```
pub const WRAP_REVERSE: Property = Property(FLEX_WRAP, "wrap-reverse");

/// ```css
/// flex-wrap: nowrap;
/// ```
pub const NOWRAP: Property = Property(FLEX_WRAP, "nowrap");
