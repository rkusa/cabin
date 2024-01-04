use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, AddAssign};

use cabin_macros::Attribute;

use super::SerializeEventFn;
use crate::error::InternalError;
use crate::html::attributes::{Attributes, WithAttribute};

pub trait Common: WithAttribute {
    /// Unique identifier across the document.
    fn id(self, id: impl Into<Cow<'static, str>>) -> Self::Output<Id> {
        self.with_attribute(Id(id.into()))
    }

    /// The various classes that the element belongs to.
    fn class(mut self, class: impl Into<Cow<'static, str>>) -> Self::Output<Class> {
        let class = if let Some(existing) = self.get_attribute_mut::<Class>() {
            Class(Cow::Owned(format!("{} {}", existing.0, class.into())))
        } else {
            Class(class.into())
        };
        self.with_attribute(class)
    }

    fn replace_class(self, class: impl Into<Cow<'static, str>>) -> Self::Output<Class> {
        self.with_attribute(Class(class.into()))
    }

    fn on_click<E>(self, event: E) -> Self::Output<OnClick>
    where
        E: serde::Serialize + 'static,
    {
        self.with_attribute(OnClick(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_click event",
                    err,
                })
                .map(|json| (hash, json))
        })))
    }

    fn on_transition_end<E>(self, event: E) -> Self::Output<OnTransitionEnd>
    where
        E: serde::Serialize + 'static,
    {
        self.with_attribute(OnTransitionEnd(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_transition_end event",
                    err,
                })
                .map(|json| (hash, json))
        })))
    }

    fn on_animation_end<E>(self, event: E) -> Self::Output<OnAnimationEnd>
    where
        E: serde::Serialize + 'static,
    {
        self.with_attribute(OnAnimationEnd(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_animation_end event",
                    err,
                })
                .map(|json| (hash, json))
        })))
    }
}

/// Unique identifier across the document.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Id(pub Cow<'static, str>);

/// The various classes that the element belongs to.
// FIXME: make it Copy
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Class(pub Cow<'static, str>);

pub struct OnClick(pub Box<SerializeEventFn>);

impl Attributes for OnClick {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        // TODO: directly write into el?
        let (id, payload) = &(self.0)()?;
        r.attribute("cabin-click", id)
            .map_err(crate::error::InternalError::from)?;
        r.attribute("cabin-click-payload", payload)
            .map_err(crate::error::InternalError::from)?;

        Ok(())
    }
}

pub struct OnTransitionEnd(pub Box<SerializeEventFn>);

impl Attributes for OnTransitionEnd {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        // TODO: directly write into el?
        let (id, payload) = &(self.0)()?;
        r.attribute("cabin-transitionend", id)
            .map_err(crate::error::InternalError::from)?;
        r.attribute("cabin-transitionend-payload", payload)
            .map_err(crate::error::InternalError::from)?;

        Ok(())
    }
}

pub struct OnAnimationEnd(pub Box<SerializeEventFn>);

impl Attributes for OnAnimationEnd {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        // TODO: directly write into el?
        let (id, payload) = &(self.0)()?;
        r.attribute("cabin-animationend", id)
            .map_err(crate::error::InternalError::from)?;
        r.attribute("cabin-animationend-payload", payload)
            .map_err(crate::error::InternalError::from)?;

        Ok(())
    }
}

impl Class {
    pub fn append(self, other: Class) -> Class {
        self + other
    }

    pub fn append_when(self, condition: bool, other: Class) -> Class {
        if !condition {
            self
        } else {
            self + other
        }
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
        // FIXME: avoid allocation
        Class(Cow::Owned(format!("{self} {rhs}")))
    }
}

impl Add<Option<Class>> for Class {
    type Output = Class;

    fn add(self, rhs: Option<Class>) -> Self::Output {
        if let Some(rhs) = rhs {
            // FIXME: avoid allocation
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
            // FIXME: avoid allocation
            Class(Cow::Owned(format!("{lhs} {rhs}")))
        } else {
            rhs
        }
    }
}

impl AddAssign for Class {
    fn add_assign(&mut self, rhs: Self) {
        // FIXME: avoid allocation
        *self = Class(Cow::Owned(format!("{self} {rhs}")))
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
