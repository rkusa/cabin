//! In Flexbox, control the alignment of items on the Cross Axis. In Grid Layout, controlc the
//! alignment of items on the Block Axis within their grid area (`align-items`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>

use crate::Property;

const ALIGN_ITEMS: &str = "align-items";

/// ```css
/// align-items: flex-start;
/// ```
pub const START: Property = Property(ALIGN_ITEMS, "flex-start");

/// ```css
/// align-items: flex-end;
/// ```
pub const END: Property = Property(ALIGN_ITEMS, "flex-end");

/// ```css
/// align-items: center
/// ```
pub const CENTER: Property = Property(ALIGN_ITEMS, "center");

/// ```css
/// align-items: baseline;
/// ```
pub const BASELINE: Property = Property(ALIGN_ITEMS, "baseline");

/// ```css
/// align-items: stretch;
/// ```
pub const STRETCH: Property = Property(ALIGN_ITEMS, "stretch");
