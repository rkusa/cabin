pub mod text;

use std::fmt;

pub use rustend_macros::css;

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
