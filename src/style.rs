use std::fmt;

pub mod length;
pub mod registry;
pub mod text;

pub trait Style {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
