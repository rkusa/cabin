use std::fmt;

use super::text::TextStyle;

pub struct Color(pub &'static str);

impl TextStyle for Color {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "color: {};", self.0)
    }
}
