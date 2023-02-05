mod class_name;
mod pseudo;
pub mod registry;
mod utilities;

use std::fmt;
use std::hash::Hasher;

pub use class_name::ClassName;
pub use rustend_macros::css;
pub use utilities::*;

pub trait Style {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result;

    fn selector_prefix(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn hash_modifier(&self, _hasher: &mut dyn Hasher) {}

    fn hover(self) -> pseudo::hover::Hover<Self>
    where
        Self: Sized,
    {
        pseudo::hover::Hover(self)
    }

    fn focus(self) -> pseudo::focus::Focus<Self>
    where
        Self: Sized,
    {
        pseudo::focus::Focus(self)
    }
}

pub struct Property<V = &'static str>(pub(crate) &'static str, pub(crate) V);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Auto,
    Px(f32),
    Rem(f32),
    Percent(f32),
}

impl Length {
    fn is_zero(&self) -> bool {
        match self {
            Length::Auto => false,
            Length::Px(v) | Length::Rem(v) | Length::Percent(v) => v.abs() < f32::EPSILON,
        }
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }

        match self {
            Length::Auto => f.write_str("auto"),
            Length::Px(v) => write!(f, "{v}px"),
            Length::Rem(v) => write!(f, "{v}rem"),
            Length::Percent(v) => write!(f, "{v}%"),
        }
    }
}

impl<V: fmt::Display> Style for Property<V> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "{}: {};", self.0, self.1)
    }
}
