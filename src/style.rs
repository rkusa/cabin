use std::fmt;

pub use text::text;

mod color;
mod font;
mod length;
pub mod preset;
pub mod registry;
mod text;

pub trait Style {
    fn css(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
