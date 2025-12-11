//! Establish an element as a query container (`container-type`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/container-type>

use crate::tailwind::Property;

const CONTAINER_TYPE: &str = "container-type";

/// ```css
/// container-type: normal;
/// ```
pub const NORMAL: Property = Property(CONTAINER_TYPE, "normal");

/// ```css
/// container-type: size;
/// ```
pub const SIZE: Property = Property(CONTAINER_TYPE, "size");

/// ```css
/// container-type: inline-size;
/// ```
pub const INLINE_SIZE: Property = Property(CONTAINER_TYPE, "inline-size");
