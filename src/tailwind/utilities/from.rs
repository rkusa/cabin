//! Set the color stops in background gradients.

use std::fmt;

use crate::tailwind::{Length, Property, Utility};

pub struct FromColor(pub(crate) &'static str);

include!(concat!(env!("OUT_DIR"), "/from-color.rs"));

/// Set a custom from color.
pub fn color(color: &'static str) -> FromColor {
    FromColor(color)
}

/// ```css
/// --tw-gradient-from-position: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property("--tw-gradient-from-position", Length::Percent(f32::from(x)))
}

/// ```css
/// --tw-gradient-from-position: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property("--tw-gradient-from-position", Length::Percent(x))
}

impl Utility for FromColor {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(
            f,
            "--tw-gradient-from: {} var(--tw-gradient-from-position);",
            self.0
        )?;
        // TODO: better handling of colors (handle non six digit hex inputs)
        writeln!(
            f,
            "--tw-gradient-to: {}00 var(--tw-gradient-to-position);",
            match self.0 {
                "inherit" | "current" => "#FFFFFF",
                "transparent" => "#000000",
                color => color,
            }
        )?;
        writeln!(
            f,
            "--tw-gradient-stops: var(--tw-gradient-from), var(--tw-gradient-to);"
        )?;
        Ok(())
    }
}
