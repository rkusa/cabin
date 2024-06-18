//! Replace the element's content (`content`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/content>

use crate::Property;

const CONTENT: &str = "--tw-content";

/// ```css
/// content: {content};
/// ```
pub fn content(content: &'static str) -> Property {
    Property(CONTENT, content)
}
