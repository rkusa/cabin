use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};

use serde::de::{DeserializeOwned, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use twox_hash::XxHash32;

use crate::scope::{event, take_event, Scope};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateId(u32);

pub struct State<T> {
    id: StateId,
    value: Option<T>,
}

impl<T> State<T>
where
    T: DeserializeOwned,
{
    pub fn id(id: impl Hash) -> Self {
        let id = StateId(hash((id, Scope::key())));
        Self {
            id,
            value: Scope::restore(id),
        }
    }
}

impl<T> State<T> {
    pub fn update<E>(mut self, update_fn: impl FnOnce(&mut T, E)) -> Self
    where
        E: DeserializeOwned + Copy + 'static,
    {
        if let Some((value, event)) = self.value.as_mut().zip(event::<E>()) {
            update_fn(value, event);
        }
        self
    }

    pub fn update_take<E>(mut self, update_fn: impl FnOnce(&mut T, E)) -> Self
    where
        E: DeserializeOwned + 'static,
    {
        if let Some((value, event)) = self.value.as_mut().zip(take_event::<E>()) {
            update_fn(value, event);
        }
        self
    }
}

impl<T> State<T>
where
    T: Serialize,
{
    pub fn restore_or(self, default: T) -> T {
        let value = self.value.unwrap_or(default);
        Scope::serialize_state(self.id, &value);
        value
    }

    pub fn restore_or_else(self, default_fn: impl FnOnce() -> T) -> T {
        let value = self.value.unwrap_or_else(default_fn);
        Scope::serialize_state(self.id, &value);
        value
    }

    pub fn restore_or_default(self) -> T
    where
        T: Default,
    {
        let value = self.value.unwrap_or_default();
        Scope::serialize_state(self.id, &value);
        value
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
        // This wrapper is used to avoid a String allocation.
        struct ForceWrite<'a> {
            out: &'a mut [u8],
            offset: usize,
        }

        impl<'a> fmt::Write for ForceWrite<'a> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.out[self.offset..self.offset + s.len()].copy_from_slice(s.as_bytes());
                self.offset += s.len();

                Ok(())
            }
        }

        let mut s = [0u8; 8];
        write!(
            ForceWrite {
                out: &mut s[..],
                offset: 0
            },
            "{:08x}",
            self.0
        )
        .unwrap();
        let id = std::str::from_utf8(&s).unwrap();
        serializer.serialize_str(id)
    }
}

impl<'de> Deserialize<'de> for StateId {
    fn deserialize<D>(deserializer: D) -> Result<StateId, D::Error>
    where
        D: Deserializer<'de>,
        D::Error: serde::de::Error,
    {
        let s = <Cow<'static, str>>::deserialize(deserializer)?;
        Ok(StateId(u32::from_str_radix(&s, 16).map_err(|_| {
            serde::de::Error::invalid_type(Unexpected::Str(&s), &"a hex encoded unsigned integer")
        })?))
    }
}
