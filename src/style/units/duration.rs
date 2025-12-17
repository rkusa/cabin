use std::fmt;
use std::hash::Hash;

use crate::style::property_display::PropertyDisplay;
use crate::style::units::float::Float;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Duration {
    Ms(u32),
    S(Float),
}

impl PropertyDisplay for Duration {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration::Ms(ms) => writeln!(f, "{name}: {ms}ms;"),
            Duration::S(s) => writeln!(f, "{name}: {s}s;"),
        }
    }
}
