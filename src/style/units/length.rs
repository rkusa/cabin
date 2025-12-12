use std::fmt;

use crate::style::property_display::PropertyDisplay;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Auto,
    MinContent,
    MaxContent,
    FitContent,
    /// Multiple of `0.25rem` (`4px` by default)
    Unit(f32),
    Vw(u16),
    Svw(u16),
    Lvw(u16),
    Dvw(u16),
    Vh(u16),
    Svh(u16),
    Lvh(u16),
    Dvh(u16),
    Px(f32),
    Em(f32),
    Rem(f32),
    Percent(f32),
    Mm(f32),
    Cm(f32),
}

impl Length {
    fn is_zero(&self) -> bool {
        match self {
            Length::Auto | Length::MinContent | Length::MaxContent | Length::FitContent => false,
            Length::Unit(v) => v.abs() < f32::EPSILON,
            Length::Vw(v)
            | Length::Svw(v)
            | Length::Lvw(v)
            | Length::Dvw(v)
            | Length::Vh(v)
            | Length::Svh(v)
            | Length::Lvh(v)
            | Length::Dvh(v) => *v == 0,
            Length::Px(v)
            | Length::Em(v)
            | Length::Rem(v)
            | Length::Percent(v)
            | Length::Mm(v)
            | Length::Cm(v) => v.abs() < f32::EPSILON,
        }
    }

    /// `0`
    pub const ZERO: Self = Self::Px(0.0);

    /// `auto`
    pub const AUTO: Self = Self::Auto;

    /// `1px`
    pub const PX: Length = Length::Px(1.0);

    /// Multiple of `0.25rem` (`4px` by default):
    /// `{x * 0.25}rem`
    pub fn unit(x: i16) -> Self {
        Length::Rem(f32::from(x) * 0.25)
    }

    /// Multiple of `0.25rem` (`4px` by default):
    /// `{x * 0.25}rem`
    pub fn unitf(x: f32) -> Self {
        Length::Rem(x * 0.25)
    }

    /// `{x}rem`
    pub fn rem(x: i16) -> Self {
        Length::Rem(f32::from(x))
    }

    /// `{x}rem`
    pub fn remf(x: f32) -> Self {
        Length::Rem(x)
    }

    /// `{x}em`
    pub fn em(x: i16) -> Self {
        Length::Em(f32::from(x))
    }

    /// `{x}em`
    pub fn emf(x: f32) -> Self {
        Length::Em(x)
    }

    /// `{x}px`
    pub fn px(x: i16) -> Self {
        Length::Px(f32::from(x))
    }

    /// `{x}px`
    pub fn pxf(x: f32) -> Self {
        Length::Px(x)
    }

    /// `{x}%`
    pub fn percent(x: i16) -> Self {
        Length::Percent(f32::from(x))
    }

    /// `{x}%`
    pub fn percentf(x: f32) -> Self {
        Length::Percent(x)
    }

    /// `{x}mm`
    pub fn mm(x: f32) -> Self {
        Length::Mm(x)
    }

    /// `{x}cm`
    pub fn cm(x: f32) -> Self {
        Length::Cm(x)
    }
}

impl From<i16> for Length {
    fn from(x: i16) -> Self {
        Length::Unit(f32::from(x))
    }
}

impl From<f32> for Length {
    fn from(x: f32) -> Self {
        Length::Unit(x)
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
            Length::Unit(v) => write!(f, "{}rem", f32::from(*v) * 0.25),
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
