//! Set how a replaced element's content should be resized (`object-fit`) and positioned within its
//! container (`object-position`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/object-position>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit>

use crate::Property;

const OBJECT_POSITION: &str = "object-position";

/// ```css
/// object-position: bottom;
/// ```
pub const BLOCK: Property = Property(OBJECT_POSITION, "bottom");

/// ```css
/// object-position: center;
/// ```
pub const CENTER: Property = Property(OBJECT_POSITION, "center");

/// ```css
/// object-position: left;
/// ```
pub const LEFT: Property = Property(OBJECT_POSITION, "left");

/// ```css
/// object-position: left bottom;
/// ```
pub const LEFT_BOTTOM: Property = Property(OBJECT_POSITION, "left bottom");

/// ```css
/// object-position: left top;
/// ```
pub const LEFT_TOP: Property = Property(OBJECT_POSITION, "left top");

/// ```css
/// object-position: right;
/// ```
pub const RIGHT: Property = Property(OBJECT_POSITION, "right");

/// ```css
/// object-position: right bottom;
/// ```
pub const RIGHT_BOTTOM: Property = Property(OBJECT_POSITION, "right bottom");

/// ```css
/// object-position: right top;
/// ```
pub const RIGHT_TOP: Property = Property(OBJECT_POSITION, "right top");

/// ```css
/// object-position: top;
/// ```
pub const TOP: Property = Property(OBJECT_POSITION, "top");

const OBJECT_FIT: &str = "object-fit";

/// ```css
/// object-fit: contain;
/// ```
pub const CONTAIN: Property = Property(OBJECT_FIT, "contain");

/// ```css
/// object-fit: cover;
/// ```
pub const COVER: Property = Property(OBJECT_FIT, "cover");

/// ```css
/// object-fit: fill;
/// ```
pub const FILL: Property = Property(OBJECT_FIT, "fill");

/// ```css
/// object-fit: none;
/// ```
pub const NONE: Property = Property(OBJECT_FIT, "none");

/// ```css
/// object-fit: scale-down;
/// ```
pub const SCALE_DOWN: Property = Property(OBJECT_FIT, "scale-down");
