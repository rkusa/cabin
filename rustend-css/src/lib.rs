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

    fn selector_suffix(&self, _f: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn hash_modifier(&self, _hasher: &mut dyn Hasher) {}

    fn override_class_name(&self) -> Option<&str> {
        None
    }

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

    fn group_hover(self) -> pseudo::group_hover::GroupHover<Self>
    where
        Self: Sized,
    {
        pseudo::group_hover::GroupHover(self)
    }
}

pub struct Property<V = &'static str>(pub(crate) &'static str, pub(crate) V);
pub struct PropertyTwice<V = &'static str>(
    pub(crate) &'static str,
    pub(crate) &'static str,
    pub(crate) V,
);

pub struct StaticClass(pub(crate) &'static str);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Auto,
    MinContent,
    MaxContent,
    FitContent,
    Vw(u16),
    Vh(u16),
    Px(f32),
    Rem(f32),
    Percent(f32),
}

impl Length {
    fn is_zero(&self) -> bool {
        match self {
            Length::Auto | Length::MinContent | Length::MaxContent | Length::FitContent => false,
            Length::Vw(v) | Length::Vh(v) => *v == 0,
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
            Length::MinContent => f.write_str("min-content"),
            Length::MaxContent => f.write_str("max-content"),
            Length::FitContent => f.write_str("fit-content"),
            Length::Vw(v) => write!(f, "{v}vw"),
            Length::Vh(v) => write!(f, "{v}vh"),
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

impl<V: fmt::Display> Style for PropertyTwice<V> {
    fn declarations(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        writeln!(f, "{}: {};", self.0, self.2)?;
        writeln!(f, "{}: {};", self.1, self.2)?;
        Ok(())
    }
}

impl Style for StaticClass {
    fn declarations(&self, _: &mut dyn fmt::Write) -> fmt::Result {
        Ok(())
    }

    fn hash_modifier(&self, hasher: &mut dyn Hasher) {
        hasher.write(self.0.as_bytes());
    }

    fn override_class_name(&self) -> Option<&str> {
        Some("group")
    }
}
