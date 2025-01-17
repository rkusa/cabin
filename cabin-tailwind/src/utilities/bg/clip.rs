//! Set whether an element's background extends underneath its border box, padding box, or content
//! box. (`background-clip`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/background-clip>

use crate::Property;

const BACKGROUND_CLIP: &str = "background-clip";

/// ```css
/// background-clip: border-box;
/// ```
pub const BORDER: Property = Property(BACKGROUND_CLIP, "border-box");

/// ```css
/// background-clip: padding-box;
/// ```
pub const PADDING: Property = Property(BACKGROUND_CLIP, "padding-box");

/// ```css
/// background-clip: content-box;
/// ```
pub const CONTENT: Property = Property(BACKGROUND_CLIP, "content-box");

/// ```css
/// background-clip: text;
/// ```
pub const TEXT: Property = Property(BACKGROUND_CLIP, "text");
