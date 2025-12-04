use core::fmt;
use std::borrow::Cow;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;
use crate::event::Event;

#[derive(Debug, Hash)]
pub struct InputValue(Cow<'static, str>);

impl Serialize for InputValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("_##InputValue")
    }
}

impl<'de> Deserialize<'de> for InputValue {
    fn deserialize<D>(deserializer: D) -> Result<InputValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Cow<'static, str> = Deserialize::deserialize(deserializer)?;
        Ok(InputValue(value))
    }
}

impl InputValue {
    pub fn placeholder() -> Self {
        Self(Cow::Borrowed(""))
    }

    pub fn take(self) -> Cow<'static, str> {
        self.0
    }
}

impl Deref for InputValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for InputValue {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<InputValue> for Cow<'static, str> {
    fn from(v: InputValue) -> Self {
        v.0
    }
}

impl fmt::Display for InputValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Hash)]
pub struct InputChecked(bool);

impl Serialize for InputChecked {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("_##InputChecked")
    }
}

impl<'de> Deserialize<'de> for InputChecked {
    fn deserialize<D>(deserializer: D) -> Result<InputChecked, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: bool = Deserialize::deserialize(deserializer)?;
        Ok(InputChecked(value))
    }
}

impl InputChecked {
    pub fn placeholder() -> Self {
        Self(false)
    }

    pub fn take(self) -> bool {
        self.0
    }
}

impl From<InputChecked> for bool {
    fn from(v: InputChecked) -> Self {
        v.0
    }
}

impl fmt::Display for InputChecked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct CustomEvent<E> {
    pub(crate) name: Cow<'static, str>,
    pub(crate) event: E,
}

impl<E: serde::Serialize + Event> CustomEvent<E> {
    pub fn new(name: impl Into<Cow<'static, str>>, event: E) -> Self {
        Self {
            name: name.into(),
            event,
        }
    }
}

impl<E: serde::Serialize + Event> Attribute for CustomEvent<E> {
    fn render(self, r: &mut crate::render::Renderer) -> Result<(), crate::Error> {
        r.event_attributes(self)
    }
}
