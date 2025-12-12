use std::fmt;

use crate::style::property_display::PropertyDisplay;

pub enum Duration {
    Ms(u32),
    S(f32),
}

impl PropertyDisplay for Duration {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Duration::Ms(ms) => writeln!(f, "{name}: {ms}ms;"),
            Duration::S(s) => writeln!(f, "{name}: {s}s;"),
        }
    }
}
