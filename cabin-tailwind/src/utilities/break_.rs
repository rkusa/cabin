//! Set how words break in an element (`word-break`/`overflow-wrap`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/word-break>
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-wrap>

use crate::{Property, PropertyTwice};

const WORD_BREAK: &str = "word-break";
const OVERFLOW_WRAP: &str = "overflow-wrap";

/// ```css
/// overflow-wrap: normal;
/// word-break: normal;
/// ```
pub const NORMAL: PropertyTwice = PropertyTwice(WORD_BREAK, OVERFLOW_WRAP, "normal");

/// ```css
/// word-break: break-wird;
/// ```
pub const WORD: Property = Property(WORD_BREAK, "break-word");

/// ```css
/// word-break: break-all;
/// ```
pub const ALL: Property = Property(WORD_BREAK, "break-all");

/// ```css
/// word-break: keep-all;
/// ```
pub const KEEP: Property = Property(WORD_BREAK, "keep-all");
