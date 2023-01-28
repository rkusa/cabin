use std::fmt;

use rustend::style::Style;

use super::Length;

pub struct TextColor(&'static str);

impl TextColor {
    pub const fn custom(color: &'static str) -> Self {
        Self(color)
    }
}

impl Style for TextColor {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "color: {};", self.0)
    }
}

pub struct FontSize {
    font_size: Length,
    line_height: Length,
}

impl FontSize {
    pub const fn custom(font_size: Length, line_height: Length) -> Self {
        Self {
            font_size,
            line_height,
        }
    }
}

impl Style for FontSize {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "font-size: {};", self.font_size)?;
        writeln!(f, "line-height: {};", self.line_height)?;
        Ok(())
    }
}
