use std::fmt;
use std::hash::Hash;

use crate::style::property_display::PropertyDisplay;
use crate::style::style_definition::MergeFrom;
use crate::style::units::float::Float;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Length {
    Auto,
    MinContent,
    MaxContent,
    FitContent,
    /// Multiple of `0.25rem` (`4px` by default)
    Unit(Float),
    Vw(u16),
    Svw(u16),
    Lvw(u16),
    Dvw(u16),
    Vh(u16),
    Svh(u16),
    Lvh(u16),
    Dvh(u16),
    Px(Float),
    Em(Float),
    Rem(Float),
    Percent(Float),
    Mm(Float),
    Cm(Float),
}

impl Length {
    fn is_zero(&self) -> bool {
        match self {
            Length::Auto | Length::MinContent | Length::MaxContent | Length::FitContent => false,
            Length::Vw(v)
            | Length::Svw(v)
            | Length::Lvw(v)
            | Length::Dvw(v)
            | Length::Vh(v)
            | Length::Svh(v)
            | Length::Lvh(v)
            | Length::Dvh(v) => *v == 0,
            Length::Unit(v)
            | Length::Px(v)
            | Length::Em(v)
            | Length::Rem(v)
            | Length::Percent(v)
            | Length::Mm(v)
            | Length::Cm(v) => v.is_zero(),
        }
    }
}

impl From<i16> for Length {
    fn from(x: i16) -> Self {
        Length::Unit(Float::from(x))
    }
}

impl From<f32> for Length {
    fn from(x: f32) -> Self {
        Length::Unit(Float::from(x))
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return f.write_str("0");
        }

        match self {
            Length::Auto => f.write_str("auto"),
            Length::MinContent => f.write_str("min-content"),
            Length::MaxContent => f.write_str("max-content"),
            Length::FitContent => f.write_str("fit-content"),
            Length::Unit(v) => write!(f, "{}rem", *v * 0.25),
            Length::Vw(v) => write!(f, "{v}vw"),
            Length::Svw(v) => write!(f, "{v}svw"),
            Length::Lvw(v) => write!(f, "{v}lvw"),
            Length::Dvw(v) => write!(f, "{v}dvw"),
            Length::Vh(v) => write!(f, "{v}vh"),
            Length::Svh(v) => write!(f, "{v}svh"),
            Length::Lvh(v) => write!(f, "{v}lvh"),
            Length::Dvh(v) => write!(f, "{v}dvh"),
            Length::Px(v) => write!(f, "{v}px"),
            Length::Em(v) => write!(f, "{v}em"),
            Length::Rem(v) => write!(f, "{v}rem"),
            Length::Percent(v) => write!(f, "{v}%"),
            Length::Mm(v) => write!(f, "{v}mm"),
            Length::Cm(v) => write!(f, "{v}cm"),
        }
    }
}

impl PropertyDisplay for Length {
    fn fmt_property(&self, name: &str, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{name}: {self};")
    }
}

impl MergeFrom for Length {
    fn merge_from(&mut self, other: Self) {
        *self = other;
    }
}
