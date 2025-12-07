use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, AddAssign};

use cabin_macros::Attribute;

use crate::attribute::WithAttribute;
use crate::event::Event;
use crate::html::events::CustomEvent;

pub trait Common: WithAttribute {
    /// Unique identifier across the document.
    fn id(self, id: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Id(id.into()))
    }

    /// The various classes that the element belongs to.
    fn class(self, class: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Class(class.into()))
    }

    fn on_click<E>(self, event: E) -> Self
    where
        E: serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("click", event))
    }

    fn on_mouse_up<E>(self, event: E) -> Self
    where
        E: serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("mouseup", event))
    }

    fn on_transition_end<E>(self, event: E) -> Self
    where
        E: serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("transitionend", event))
    }

    fn on_animation_end<E>(self, event: E) -> Self
    where
        E: serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("animationend", event))
    }
}

/// Unique identifier across the document.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Id(pub Cow<'static, str>);

/// The various classes that the element belongs to.
// TODO: make it Copy
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Class(pub Cow<'static, str>);

impl Class {
    pub fn append(self, other: Class) -> Class {
        self + other
    }

    pub fn append_when(self, condition: bool, other: Class) -> Class {
        if !condition { self } else { self + other }
    }
}

impl Default for Class {
    fn default() -> Self {
        Class(Cow::Borrowed(""))
    }
}

impl From<Class> for Cow<'static, str> {
    fn from(value: Class) -> Self {
        value.0
    }
}

impl From<Option<&'static str>> for Class {
    fn from(value: Option<&'static str>) -> Self {
        Class(value.map(Cow::Borrowed).unwrap_or(Cow::Borrowed("")))
    }
}

impl Add<Class> for Class {
    type Output = Class;

    fn add(self, rhs: Class) -> Self::Output {
        // TODO: avoid allocation
        Class(Cow::Owned(format!("{self} {rhs}")))
    }
}

impl Add<Option<Class>> for Class {
    type Output = Class;

    fn add(self, rhs: Option<Class>) -> Self::Output {
        if let Some(rhs) = rhs {
            // TODO: avoid allocation
            Class(Cow::Owned(format!("{self} {rhs}")))
        } else {
            self
        }
    }
}

impl Add<Class> for Option<Class> {
    type Output = Class;

    fn add(self, rhs: Class) -> Self::Output {
        if let Some(lhs) = self {
            // TODO: avoid allocation
            Class(Cow::Owned(format!("{lhs} {rhs}")))
        } else {
            rhs
        }
    }
}

impl AddAssign for Class {
    fn add_assign(&mut self, rhs: Self) {
        // TODO: avoid allocation
        *self = Class(Cow::Owned(format!("{self} {rhs}")))
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
