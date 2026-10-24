use std::fmt;

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;
use crate::style::units::length::Length;

/// The individual `translate` property, so translations compose with each other and with
/// `rotate`, instead of replacing each other like `transform` functions would.
#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct Translate {
    pub x: Option<Length>,
    pub y: Option<Length>,
}

impl PropertyDisplay for Translate {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.x, &self.y) {
            (Some(x), Some(y)) => writeln!(f, "{name}: {x} {y};"),
            (Some(x), None) => writeln!(f, "{name}: {x} 0;"),
            (None, Some(y)) => writeln!(f, "{name}: 0 {y};"),
            (None, None) => Ok(()),
        }
    }
}

impl MergeFrom for Translate {
    fn merge_from(&mut self, other: Self) {
        let Self { x, y } = other;
        self.x.merge_from(x);
        self.y.merge_from(y);
    }
}

/// An angle in degrees.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Deg(pub i16);

impl PropertyDisplay for Deg {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{name}: {}deg;", self.0)
    }
}

impl MergeFrom for Deg {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}
