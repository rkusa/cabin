//! Set the bullet/number style of a list (`list-style-type`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/list-style-type>

use crate::Property;

const LIST_STYLE_TYPE: &str = "list-style-type";

/// ```css
/// list-style-type: none;
/// ```
pub const NONE: Property = Property(LIST_STYLE_TYPE, "none");

/// ```css
/// list-style-type: disc;
/// ```
pub const DISC: Property = Property(LIST_STYLE_TYPE, "disc");

/// ```css
/// list-style-type: decimnal;
/// ```
pub const DECIMAL: Property = Property(LIST_STYLE_TYPE, "decimnal");
