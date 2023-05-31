use std::ops::{Deref, DerefMut};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::scope::Scope;

pub type SignalId = &'static str;

pub struct Signal<T>
where
    T: Serialize,
{
    id: SignalId,
    value: T,
}

impl<T> Signal<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn new(id: SignalId, default: T) -> Self {
        Self {
            id,
            value: Scope::restore(id).unwrap_or(default),
        }
    }
}

impl<T> Signal<T>
where
    T: Serialize,
{
    pub(crate) fn id(&self) -> SignalId {
        self.id
    }

    pub(crate) fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Deref for Signal<T>
where
    T: Serialize,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Scope::add_signal(self);
        &self.value
    }
}

impl<T> DerefMut for Signal<T>
where
    T: Serialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Drop for Signal<T>
where
    T: Serialize,
{
    fn drop(&mut self) {
        Scope::serialize_signal(self);
    }
}
