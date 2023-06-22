//! Define how to distribute space between and around content items along the main-axis of a flex
//! container, and the inline axis of a grid container (`justify-content`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>

use crate::Property;

const JUSTIFY_CONTENT: &str = "justify-content";

/// `justify-content: normal;`
pub const NORMAL: Property = Property(JUSTIFY_CONTENT, "normal");

/// `justify-content: flex-start;`
pub const START: Property = Property(JUSTIFY_CONTENT, "flex-start");

/// `justify-content: flex-end;`
pub const END: Property = Property(JUSTIFY_CONTENT, "flex-end");

/// `justify-content: center;`
pub const CENTER: Property = Property(JUSTIFY_CONTENT, "center");

/// `justify-content: space-between;`
pub const BETWEEN: Property = Property(JUSTIFY_CONTENT, "space-between");

/// `justify-content: space-around;`
pub const AROUND: Property = Property(JUSTIFY_CONTENT, "space-around");

/// `justify-content: space-evenly;`
pub const EVENLY: Property = Property(JUSTIFY_CONTENT, "space-evenly");

/// `justify-content: stretch;`
pub const STRETCH: Property = Property(JUSTIFY_CONTENT, "stretch");
