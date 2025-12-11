//! Add a shadow effects around an element's frame (`box-shadow`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/box-shadow>

use std::fmt;

use crate::tailwind::{Property, Utility};

const SHADOW_COLOR: &str = "--tw-shadow-color";

include!(concat!(env!("OUT_DIR"), "/shadow-color.rs"));

/// Set a custom border color.
pub fn color(color: &'static str) -> Property {
    Property(SHADOW_COLOR, color)
}

pub struct BoxShadow(pub &'static str, pub &'static str);

pub const SM: BoxShadow = BoxShadow(
    "0 1px 2px 0 rgb(0 0 0 / .05)",
    "0 1px 2px 0 var(--tw-shadow-color)",
);

pub const DEFAULT: BoxShadow = BoxShadow(
    "0 1px 3px 0 rgb(0 0 0 / .1), 0 1px 2px -1px rgb(0 0 0 / .1)",
    "0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color)",
);

pub const MD: BoxShadow = BoxShadow(
    "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
    "0 4px 6px -1px var(--tw-shadow-color), 0 2px 4px -2px var(--tw-shadow-color)",
);

pub const LG: BoxShadow = BoxShadow(
    "0 10px 15px -3px rgb(0 0 0 / .1), 0 4px 6px -4px rgb(0 0 0 / .1)",
    "0 10px 15px -3px var(--tw-shadow-color), 0 4px 6px -4px var(--tw-shadow-color)",
);

pub const XL: BoxShadow = BoxShadow(
    "0 20px 25px -5px rgb(0 0 0 / .1), 0 8px 10px -6px rgb(0 0 0 / .1)",
    "0 20px 25px -5px var(--tw-shadow-color), 0 8px 10px -6px var(--tw-shadow-color)",
);

pub const XL2: BoxShadow = BoxShadow(
    "0 25px 50px -12px rgb(0 0 0 / .25)",
    "0 25px 50px -12px var(--tw-shadow-color)",
);

pub const INNER: BoxShadow = BoxShadow(
    "inset 0 2px 4px 0 rgb(0 0 0 / .05)",
    "inset 0 2px 4px 0 var(--tw-shadow-color)",
);

pub const NONE: BoxShadow = BoxShadow("0 0 #0000", "0 0 #0000");

impl Utility for BoxShadow {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "--tw-shadow: {};", self.0)?;
        write!(f, "--tw-ring-shadow: {};", self.1)?;
        write!(
            f,
            "box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 \
             #0000), var(--tw-shadow);"
        )?;
        Ok(())
    }
}
