use std::borrow::Cow;

use cabin_macros::Attribute;

use crate::event::Event;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::events::CustomEvent;
use crate::tailwind::Tailwind;

pub trait Common: WithAttribute {
    /// Unique identifier across the document.
    fn id(self, id: impl Into<Cow<'static, str>>) -> Self::Output<Id> {
        self.with_attribute(Id(id.into()))
    }

    /// The various classes that the element belongs to.
    fn class(mut self, class: impl Into<Class>) -> Self::Output<Class> {
        let class = if let Some(existing) = self.get_attribute_mut::<Class>() {
            let existing = std::mem::take(existing);
            existing.append(class.into())
        } else {
            class.into()
        };
        self.with_attribute(class)
    }

    /// Add the `group` class that can be used in combination with
    /// [crate::style::Style::group_hover()].
    fn group(mut self) -> Self::Output<Class> {
        let class = if let Some(existing) = self.get_attribute_mut::<Class>() {
            let existing = std::mem::take(existing);
            existing.append(Class::from("group"))
        } else {
            Class::from("group")
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
pub struct Class {
    pub class: Cow<'static, str>,
    pub tailwind: Tailwind,
}

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
    pub fn append(self, other: impl Into<Class>) -> Class {
        let other = other.into();
        Class {
            class: if self.class.is_empty() {
                other.class
            } else if other.class.is_empty() {
                self.class
            } else {
                Cow::Owned(format!("{} {}", self.class, other.class))
            },
            tailwind: self.tailwind.append(other.tailwind),
        }
    }

    pub fn append_when(self, condition: bool, other: impl Into<Class>) -> Class {
        if !condition { self } else { self.append(other) }
    }
}

impl Default for Class {
    fn default() -> Self {
        Class {
            class: Cow::Borrowed(""),
            tailwind: Default::default(),
        }
    }
}

impl From<&'static str> for Class {
    fn from(class: &'static str) -> Self {
        Class {
            class: Cow::Borrowed(class),
            tailwind: Default::default(),
        }
    }
}

impl From<Cow<'static, str>> for Class {
    fn from(class: Cow<'static, str>) -> Self {
        Class {
            class,
            tailwind: Default::default(),
        }
    }
}

impl From<Tailwind> for Class {
    fn from(tailwind: Tailwind) -> Self {
        Class {
            class: Cow::Borrowed(""),
            tailwind,
        }
    }
}

impl From<Option<&'static str>> for Class {
    fn from(value: Option<&'static str>) -> Self {
        Class {
            class: value.map(Cow::Borrowed).unwrap_or(Cow::Borrowed("")),
            tailwind: Default::default(),
        }
    }
}

impl Attributes for Class {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        if self.class.is_empty() && self.tailwind.is_empty() {
            return Ok(());
        }

        if self.tailwind.is_empty() {
            r.attribute("class", self.class);
        } else {
            let tailwind_classes = self.tailwind.append_to(&mut r.renderer.styles);
            r.attribute("class", format!("{} {}", self.class, tailwind_classes));
        }
        Ok(())
    }
}
