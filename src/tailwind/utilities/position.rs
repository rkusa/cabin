//! Set how an element is positioned in a document (`position`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/position>

use crate::tailwind::Property;

const POSITION: &str = "position";

/// ```css
/// position: static;
/// ```
pub const STATIC: Property = Property(POSITION, "static");

/// ```css
/// position: relative;
/// ```
pub const RELATIVE: Property = Property(POSITION, "relative");

/// ```css
/// position: absolute;
/// ```
pub const ABSOLUTE: Property = Property(POSITION, "absolute");

/// ```css
/// position: sticky;
/// ```
pub const STICKY: Property = Property(POSITION, "sticky");

/// ```css
/// position: fixed;
/// ```
pub const FIXED: Property = Property(POSITION, "fixed");
