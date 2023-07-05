//! Set the color stops in background gradients.

use std::fmt;

use crate::{Length, Property, Style};

pub struct ViaColor(pub(crate) &'static str);

include!(concat!(env!("OUT_DIR"), "/via-color.rs"));

/// ```css
/// --tw-gradient-via-position: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property("--tw-gradient-via-position", Length::Percent(f32::from(x)))
}

/// ```css
/// --tw-gradient-via-position: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property("--tw-gradient-via-position", Length::Percent(x))
}

impl Style for ViaColor {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
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
            "--tw-gradient-stops: var(--tw-gradient-from), inherit var(--tw-gradient-via-position), var(--tw-gradient-to);"
        )?;
        Ok(())
    }
}
