//! Control which CSS properties transition (`transition-property`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/transition-property>

use std::fmt;

use crate::{Property, Utility};

pub struct Transition(pub &'static str);

/// Set which properties to transition.
/// ```css
/// transition-property: {property};
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// transition-duration: 150ms;
/// ```
pub fn property(property: &'static str) -> Transition {
    Transition(property)
}

/// ```css
/// transition-property: none;
/// ```
pub const NONE: Property = Property("transition-property", "none");

/// ```css
/// transition-property: all;
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// transition-duration: 150ms;
/// ```
pub const ALL: Transition = Transition("all");

/// ```css
/// transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// transition-duration: 150ms;
/// ```
pub const DEFAULT: Transition = Transition(
    "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, \
     box-shadow, transform, filter, backdrop-filter",
);

/// ```css
/// transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// transition-duration: 150ms;
/// ```
pub const COLORS: Transition =
    Transition("color, background-color, border-color, text-decoration-color, fill, stroke");

/// ```css
/// transition-property: opacity;
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// transition-duration: 150ms;
/// ```
pub const OPACITY: Transition = Transition("opacity");

/// ```css
/// transition-property: box-shadow;
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// transition-duration: 150ms;
/// ```
pub const SHADOW: Transition = Transition("box-shadow");

/// ```css
/// transition-property: transform;
/// transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
/// transition-duration: 150ms;
/// ```
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

    fn order(&self) -> usize {
        0
    }
}

pub struct DurationMs(pub u32);

/// Duration of CSS transitions in milliseconds.
/// ```css
/// transition-duration: {ms}ms;
/// ```
pub fn duration_ms(ms: u32) -> DurationMs {
    DurationMs(ms)
}

impl Utility for DurationMs {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transition-duration: {}ms;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct DurationS(pub f32);

/// Duration of CSS transitions in seconds.
/// ```css
/// transition-duration: {s}s;
/// ```
pub fn duration_s(s: f32) -> DurationS {
    DurationS(s)
}

impl Utility for DurationS {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transition-duration: {}s;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct DelayMs(pub u32);

/// Delay of CSS transitions in milliseconds.
/// ```css
/// transition-delay: {ms}ms;
/// ```
pub fn delay_ms(ms: u32) -> DelayMs {
    DelayMs(ms)
}

impl Utility for DelayMs {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transition-delay: {}ms;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}

pub struct DelayS(pub f32);

/// Delay of CSS transitions in seconds.
/// ```css
/// transition-delay: {s}s;
/// ```
pub fn delay_s(s: f32) -> DelayS {
    DelayS(s)
}

impl Utility for DelayS {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "transition-delay: {}s;", self.0)?;
        Ok(())
    }

    fn order(&self) -> usize {
        1
    }
}
