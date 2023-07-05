//! Control the style of an element's outline (`outline-style`).
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/outline-style>

use std::fmt;

pub use offset::width as offset;

use crate::{Length, Property, Style};

const BOX_SHADOW: &str = "box-shadow";

pub struct RingColor<V = &'static str>(pub(crate) V);

include!(concat!(env!("OUT_DIR"), "/ring-color.rs"));

pub mod offset {
    use super::*;

    pub struct RingOffsetColor<V = &'static str>(pub(crate) V);

    include!(concat!(env!("OUT_DIR"), "/ring-offset-color.rs"));

    /// ```css
    /// --tw-ring-offset-width: {x}px;
    /// box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    /// ```
    pub fn width(x: i16) -> RingOffsetWidth {
        RingOffsetWidth(Length::Px(f32::from(x)))
    }

    /// ```css
    /// --tw-ring-offset-width: {x}px;
    /// box-shadow: 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color), var(--tw-ring-shadow);
    /// ```
    pub fn widthf(x: f32) -> RingOffsetWidth {
        RingOffsetWidth(Length::Px(x))
    }

    pub struct RingOffsetWidth(pub Length);

    impl Style for RingOffsetWidth {
        fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
            write!(f, "--tw-ring-offset-width: {};", self.0)?;
            write!(
                f,
                "box-shadow: 0 0 0 var(--tw-ring-offset-width) \
                var(--tw-ring-offset-color), var(--tw-ring-shadow);",
            )?;
            Ok(())
        }
    }

    impl<V: fmt::Display> Style for RingOffsetColor<V> {
        fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
            writeln!(f, "--tw-ring-offset-color: {};", self.0)?;
            writeln!(
                f,
                "box-shadow: 0 0 0 var(--tw-ring-offset-width) \
                 var(--tw-ring-offset-color), var(--tw-ring-shadow);"
            )?;
            Ok(())
        }
    }
}

/// Force a ring to render on the inside of an element instead of the outside.
/// ```css
/// --tw-ring-inset: inset;
/// ```
pub const INSET: Property = Property("--tw-ring-inset", "inset");

pub const DEFAULT: Property<RingWidth> = Property(BOX_SHADOW, RingWidth(Length::Px(3.0)));

/// ```css
/// box-shadow: var(--tw-ring-inset) 0 0 0 calc({x}px + var(--tw-ring-offset-width)) var(--tw-ring-color);
/// ```
pub fn width(x: i16) -> RingWidth {
    RingWidth(Length::Px(f32::from(x)))
}

/// ```css
/// box-shadow: var(--tw-ring-inset) 0 0 0 calc({x}px + var(--tw-ring-offset-width)) var(--tw-ring-color);
/// ```
pub fn widthf(x: f32) -> RingWidth {
    RingWidth(Length::Px(x))
}

pub struct RingWidth(pub Length);

impl Style for RingWidth {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "--tw-ring-offset-shadow: var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);")?;
        write!(f, "--tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc({} + var(--tw-ring-offset-width)) var(--tw-ring-color);", self.0)?;
        write!(f, "box-shadow: var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000);")?;
        Ok(())
    }
}

impl<V: fmt::Display> Style for RingColor<V> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "--tw-ring-color: {};", self.0)?;
        Ok(())
    }
}
