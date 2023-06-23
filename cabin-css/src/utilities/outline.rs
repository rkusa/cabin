//! Control the style of an element's outline (`outline-style`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style>

use std::fmt;

use crate::{Length, Property, Style};

const OUTLINE_STYLE: &str = "outline-style";
const OUTLINE_WIDTH: &str = "outline-width";
const OUTLINE_OFFSET: &str = "outline-offset";
const OUTLINE_COLOR: &str = "outline-color";

include!(concat!(env!("OUT_DIR"), "/outline-color.rs"));

/// Hide the default browser outline on focused elements.
/// ```css
/// outline: 2px solid transparent;
/// outline-offset: 2px;
/// ```
pub const NONE: OutlineNone = OutlineNone;

/// `outline-style: solid;`
pub const SOLID: Property = Property(OUTLINE_STYLE, "solid");

/// `outline-style: dashed;`
pub const DASHED: Property = Property(OUTLINE_STYLE, "dashed");

/// `outline-style: dotted;`
pub const DOTTED: Property = Property(OUTLINE_STYLE, "dotted");

/// `outline-style: double;`
pub const DOUBLE: Property = Property(OUTLINE_STYLE, "double");

/// `outline-offset: {x}px;`
pub fn offset(x: i16) -> Property<Length> {
    Property(OUTLINE_OFFSET, Length::Px(f32::from(x)))
}

/// `outline-offset: {x}px;`
pub fn offsetf(x: f32) -> Property<Length> {
    Property(OUTLINE_OFFSET, Length::Px(x))
}

/// `outline-width: {x}px;`
pub fn width(x: i16) -> Property<Length> {
    Property(OUTLINE_WIDTH, Length::Px(f32::from(x)))
}

/// `outline-width: {x}px;`
pub fn widthf(x: f32) -> Property<Length> {
    Property(OUTLINE_WIDTH, Length::Px(x))
}

pub struct OutlineNone;

impl Style for OutlineNone {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "outline: 2px solid transparent;")?;
        writeln!(f, "outline-offset: 2px;")?;
        Ok(())
    }
}
