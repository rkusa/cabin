//! Set the foreground color value of an element's text and text decorations.
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/color>

use core::fmt;

use crate::{Length, Property, Style};

const COLOR: &str = "color";

include!(concat!(env!("OUT_DIR"), "/text-color.rs"));

/// Set a custom foreground color.
pub fn color(color: &'static str) -> Property {
    Property(COLOR, color)
}

include!(concat!(env!("OUT_DIR"), "/font-size.rs"));

pub struct FontSize {
    font_size: Length,
    line_height: LineHeight,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineHeight {
    Length(Length),
    Multiple(u16),
}

/// Set a custom `font-size` and `line-height`.
pub const fn size(font_size: Length, line_height: LineHeight) -> FontSize {
    FontSize {
        font_size,
        line_height,
    }
}

impl Style for FontSize {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "font-size: {};", self.font_size)?;
        writeln!(f, "line-height: {};", self.line_height)?;
        Ok(())
    }
}

impl fmt::Display for LineHeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineHeight::Length(l) => l.fmt(f),
            LineHeight::Multiple(x) => write!(f, "{x}"),
        }
    }
}
