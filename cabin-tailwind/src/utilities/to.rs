//! Set the color stops in background gradients.

use std::fmt;

use crate::{Length, Property, Utility};

pub struct ToColor(pub(crate) &'static str);

include!(concat!(env!("OUT_DIR"), "/to-color.rs"));

/// ```css
/// --tw-gradient-to-position: {x}%;
/// ```
pub fn percent(x: i16) -> Property<Length> {
    Property("--tw-gradient-to-position", Length::Percent(f32::from(x)))
}

/// ```css
/// --tw-gradient-to-position: {x}%;
/// ```
pub fn percentf(x: f32) -> Property<Length> {
    Property("--tw-gradient-to-position", Length::Percent(x))
}

impl Utility for ToColor {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        // TODO: better handling of colors (handle non six digit hex inputs)
        writeln!(
            f,
            "--tw-gradient-to: {}FF var(--tw-gradient-to-position);",
            self.0
        )?;
        Ok(())
    }

    fn order(&self) -> usize {
        // Take precedence over from color.
        1
    }
}
