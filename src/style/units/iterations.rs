use std::fmt;

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Iterations {
    Count(u16),
    Infinite,
}

impl PropertyDisplay for Iterations {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Iterations::Count(count) => writeln!(f, "{name}: {count};"),
            Iterations::Infinite => writeln!(f, "{name}: infinite;"),
        }
    }
}

impl MergeFrom for Iterations {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}
