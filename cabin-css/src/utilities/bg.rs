//! Set the background color an element (`background-color`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/background-color>

use crate::Property;

const BACKGROUND_COLOR: &str = "background-color";

include!(concat!(env!("OUT_DIR"), "/background-color.rs"));

/// Set a custom background color.
pub fn color(color: &'static str) -> Property {
    Property(BACKGROUND_COLOR, color)
}
