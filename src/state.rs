use std::borrow::Cow;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use serde::de::{DeserializeOwned, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use twox_hash::XxHash32;

use crate::scope::Scope;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateId(u32);

pub struct State<T>
where
    T: Serialize,
{
    id: StateId,
    value: Option<T>,
}

impl<T> State<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn restore_or(id: impl Hash, default: T) -> Self {
        let id = StateId(hash((id, Scope::key())));
        Self {
            id,
            value: Some(Scope::restore(id).unwrap_or(default)),
        }
    }

    pub fn restore_or_else(id: impl Hash, default: impl FnOnce() -> T) -> Self {
        let id = StateId(hash((id, Scope::key())));
        Self {
            id,
            value: Some(Scope::restore(id).unwrap_or_else(default)),
        }
    }
}

impl<T> State<T>
where
    T: Serialize,
{
    pub(crate) fn id(&self) -> StateId {
        self.id
    }

    pub(crate) fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

impl<T> Clone for State<T>
where
    T: Serialize + Clone,
{
    fn clone(&self) -> Self {
        State {
            id: self.id,
            value: self.value.clone(),
        }
    }
}

impl<T> Deref for State<T>
where
    T: Serialize,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Scope::add_state(self);
        self.value.as_ref().unwrap()
    }
}

impl<T> DerefMut for State<T>
where
    T: Serialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<T> Drop for State<T>
where
    T: Serialize,
{
    fn drop(&mut self) {
        // TODO: not going to work for static signals
        Scope::serialize_state(self);
    }
}

impl<T> Display for State<T>
where
    T: Serialize + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.as_ref().unwrap().fmt(f)
    }
}

impl<T> IntoIterator for State<T>
where
    T: Serialize + IntoIterator,
{
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    fn into_iter(mut self) -> Self::IntoIter {
        Scope::serialize_state(&self);
        self.value.take().unwrap().into_iter()
    }
}

fn hash(val: impl Hash) -> u32 {
    let mut hasher = XxHash32::default();
    val.hash(&mut hasher);
    hasher.finish() as u32
}

impl Serialize for StateId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO: any way around String allocation?
        serializer.serialize_str(&format!("{:x}", self.0))
    }
}

impl<'de> Deserialize<'de> for StateId {
    fn deserialize<D>(deserializer: D) -> Result<StateId, D::Error>
    where
        D: Deserializer<'de>,
        D::Error: serde::de::Error,
    {
        let s = <&str>::deserialize(deserializer)?;
        Ok(StateId(u32::from_str_radix(s, 16).map_err(|_| {
            serde::de::Error::invalid_type(Unexpected::Str(s), &"a hex encoded unsigned integer")
        })?))
    }
}

impl<'a> From<State<Cow<'a, str>>> for Cow<'a, str> {
    fn from(mut value: State<Cow<'a, str>>) -> Self {
        Scope::serialize_state(&value);
        value.value.take().unwrap()
    }
}
