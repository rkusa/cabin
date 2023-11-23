//! Control which CSS properties transition (`transition-property`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-property>

use std::fmt;

use crate::{Property, Utility};

pub struct Transition(pub &'static str);

pub const NONE: Property = Property("transition-property", "none");

pub const ALL: Transition = Transition("all");

pub const DEFAULT: Transition = Transition("color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter");

pub const COLORS: Transition =
    Transition("color, background-color, border-color, text-decoration-color, fill, stroke");

pub const OPACITY: Transition = Transition("opacity");

pub const SHADOW: Transition = Transition("box-shadow");

pub const TRANSFORM: Transition = Transition("transform");

impl Utility for Transition {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transition-property: {};", self.0)?;
        write!(
            f,
            "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
        )?;
        write!(f, "transition-duration: 150ms;")?;
        Ok(())
    }
}
