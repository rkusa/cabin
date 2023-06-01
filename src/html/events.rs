use core::fmt;
use std::borrow::Cow;
use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Default)]
#[non_exhaustive]
pub struct InputEvent {
    pub value: InputValue,
}

#[derive(Default, Hash)]
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
