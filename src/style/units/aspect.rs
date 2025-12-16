use std::fmt;

use crate::style::property_display::PropertyDisplay;

#[derive(Clone)]
pub enum Aspect {
    Auto,
    Ratio(u32, u32),
    Ratiof(f32),
}

impl PropertyDisplay for Aspect {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Aspect::Auto => writeln!(f, "{name}: auto;"),
            Aspect::Ratio(w, h) => writeln!(f, "{name}: {w} / {h};"),
            Aspect::Ratiof(r) => writeln!(f, "{name}: {r};"),
        }
    }
}
