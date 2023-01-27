use std::fmt;

use super::length::Length;
use super::text::TextStyle;

pub struct FontSize {
    pub font_size: Length,
    pub line_height: Length,
}

impl TextStyle for FontSize {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "font-size: {};", self.font_size)?;
        writeln!(f, "line-height: {};", self.line_height)?;
        Ok(())
    }
}
