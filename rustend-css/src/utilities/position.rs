//! Set how an element is positioned in a document (`position`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/position>

use crate::Property;

const POSITION: &str = "position";

/// ```
/// position: static;
/// ```
pub const STATIC: Property = Property(POSITION, "static");

/// ```
/// position: relative;
/// ```
pub const RELATIVE: Property = Property(POSITION, "relative");

/// ```
/// position: absolute;
/// ```
pub const ABSOLUTE: Property = Property(POSITION, "absolute");

/// ```
/// position: sticky;
/// ```
pub const STICKY: Property = Property(POSITION, "sticky");

/// ```
/// position: fixed;
/// ```
pub const FIXED: Property = Property(POSITION, "fixed");
