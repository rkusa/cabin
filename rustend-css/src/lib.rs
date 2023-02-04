pub mod bg;
mod class_name;
mod pseudo;
pub mod registry;
pub mod text;

use std::fmt;
use std::hash::Hasher;

pub use class_name::ClassName;
pub use rustend_macros::css;
pub use utilities::*;

pub mod utilities {
    use super::{Length, Style};

    include!(concat!(env!("OUT_DIR"), "/rustend-css-build.rs"));
}

pub trait Style {
    fn declarations(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn selector_prefix(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub enum Length {
    Auto,
    Px(f32),
    Rem(f32),
    Percent(f32),
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Auto => f.write_str("auto"),
            Length::Px(v) => write!(f, "{v}px"),
            Length::Rem(v) => write!(f, "{v}rem"),
            Length::Percent(v) => write!(f, "{v}%"),
        }
    }
}
