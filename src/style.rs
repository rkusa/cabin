use std::fmt;

pub mod registry;

pub trait Style {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
