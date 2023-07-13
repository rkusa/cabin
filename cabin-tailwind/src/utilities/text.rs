//! Utilities for controlling text layout, behaviour and color, including:
//!
//! - `color` <https://developer.mozilla.org/en-US/docs/Web/CSS/color>
//! - `font-size` <https://developer.mozilla.org/en-US/docs/Web/CSS/font-size>
//! - `line-height` <https://developer.mozilla.org/en-US/docs/Web/CSS/line-height>
//! - `text-align` <https://developer.mozilla.org/en-US/docs/Web/CSS/text-align>

use core::fmt;

use crate::{Length, Property, Utility};

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

impl Utility for FontSize {
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

const TEXT_ALIGN: &str = "text-align";

/// ```css
/// text-align: left;
/// ```
pub const LEFT: Property = Property(TEXT_ALIGN, "left");

/// ```css
/// text-align: center;
/// ```
pub const CENTER: Property = Property(TEXT_ALIGN, "center");

/// ```css
/// text-align: right;
/// ```
pub const RIGHT: Property = Property(TEXT_ALIGN, "right");

/// ```css
/// text-align: justify;
/// ```
pub const JUSTIFY: Property = Property(TEXT_ALIGN, "justify");

/// ```css
/// text-align: start;
/// ```
pub const START: Property = Property(TEXT_ALIGN, "start");

/// ```css
/// text-align: end;
/// ```
pub const END: Property = Property(TEXT_ALIGN, "end");
