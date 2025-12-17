use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use crate::event::Event;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::events::CustomEvent;

pub trait Common: WithAttribute {
    /// Unique identifier across the document.
    fn id(self, id: impl Into<Cow<'static, str>>) -> Self::Output<Id> {
        self.with_attribute(Id(id.into()))
    }

    /// The various classes that the element belongs to.
    fn class(mut self, class: impl Into<Cow<'static, str>>) -> Self::Output<Class> {
        let class = if let Some(existing) = self.get_attribute_mut::<Class>() {
            let existing = std::mem::take(existing);
            existing.append(Class(class.into()))
        } else {
            Class(class.into())
        };
        self.with_attribute(class)
    }

    /// Add the `group` class that can be used in combination with
    /// [crate::style::Style::group_hover()].
    fn group(mut self) -> Self::Output<Class> {
        let class = if let Some(existing) = self.get_attribute_mut::<Class>() {
            let existing = std::mem::take(existing);
            existing.append(Class("group".into()))
        } else {
            Class("group".into())
        };
        self.with_attribute(class)
    }

    fn on_click<E>(self, event: E) -> Self::Output<OnClick<E>>
    where
        E: serde::Serialize + Event + Send + 'static,
    {
        self.with_attribute(OnClick(CustomEvent::new("click", event)))
    }

    fn on_mouse_up<E>(self, event: E) -> Self::Output<OnMouseUp<E>>
    where
        E: serde::Serialize + Event + Send + 'static,
    {
        self.with_attribute(OnMouseUp(CustomEvent::new("mouseup", event)))
    }

    fn on_transition_end<E>(self, event: E) -> Self::Output<OnTransitionEnd<E>>
    where
        E: serde::Serialize + Event + Send + 'static,
    {
        self.with_attribute(OnTransitionEnd(CustomEvent::new("transitionend", event)))
    }

    fn on_animation_end<E>(self, event: E) -> Self::Output<OnAnimationEnd<E>>
    where
        E: serde::Serialize + Event + Send + 'static,
    {
        self.with_attribute(OnAnimationEnd(CustomEvent::new("animationend", event)))
    }
}

/// Unique identifier across the document.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Id(pub Cow<'static, str>);

/// The various classes that the element belongs to.
// FIXME: make it Copy
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Class(pub Cow<'static, str>);

pub struct OnClick<E>(CustomEvent<E>);

impl<E: serde::Serialize + Event + Send + 'static> Attributes for OnClick<E> {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        self.0.render(r)
    }
}

pub struct OnMouseUp<E>(CustomEvent<E>);

impl<E: serde::Serialize + Event + Send + 'static> Attributes for OnMouseUp<E> {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        self.0.render(r)
    }
}

pub struct OnTransitionEnd<E>(CustomEvent<E>);

impl<E: serde::Serialize + Event + Send + 'static> Attributes for OnTransitionEnd<E> {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        self.0.render(r)
    }
}

pub struct OnAnimationEnd<E>(CustomEvent<E>);

impl<E: serde::Serialize + Event + Send + 'static> Attributes for OnAnimationEnd<E> {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        self.0.render(r)
    }
}

impl Class {
    pub fn append(self, other: Class) -> Class {
        if self.0.is_empty() {
            return other;
        } else if other.0.is_empty() {
            return self;
        }
        // FIXME: avoid allocation
        match self.0 {
            Cow::Borrowed(s) => Class(Cow::Owned(format!("{s} {other}"))),
            Cow::Owned(mut s) => {
                if !s.is_empty() {
                    s += " ";
                }
                s += &other.0;
                Class(Cow::Owned(s))
            }
        }
    }

    pub fn append_when(self, condition: bool, other: Class) -> Class {
        if !condition { self } else { self.append(other) }
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

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
