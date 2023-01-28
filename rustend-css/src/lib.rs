pub mod bg;
mod class_name;
pub mod display;
mod pseudo;
pub mod registry;
pub mod text;

use std::fmt;
use std::hash::Hasher;

pub use class_name::ClassName;
pub use display::*;
pub use rustend_macros::css;

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
    Px(f32),
    Rem(f32),
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Length::Px(v) => write!(f, "{v}px"),
            Length::Rem(v) => write!(f, "{v}rem"),
        }
    }
}
