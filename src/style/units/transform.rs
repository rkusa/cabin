use std::fmt;

use crate::style::property_display::PropertyDisplay;
use crate::style::units::length::Length;

#[derive(Clone)]
pub enum Transform {
    Rotate(i16),
    TranslateX(Length),
    TranslateY(Length),
}

impl PropertyDisplay for Transform {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Transform::Rotate(deg) => writeln!(f, "{name}: rotate({deg}deg);"),
            Transform::TranslateX(x) => writeln!(f, "{name}: translateX({x});"),
            Transform::TranslateY(x) => writeln!(f, "{name}: translateY({x});"),
        }
    }
}
