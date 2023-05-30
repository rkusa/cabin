use core::fmt;
use std::ops::{Deref, DerefMut};

use serde::Serialize;

use crate::scope::Scope;

pub type ScopeId = &'static str;
pub type SignalId = &'static str;

pub struct Signal<T>
where
    T: Serialize,
{
    scope_id: ScopeId,
    id: SignalId,
    value: T,
}

pub struct SignalMut<T>(Signal<T>)
where
    T: Serialize;

impl<T> Signal<T>
where
    T: Serialize,
{
    pub fn new(scope_id: ScopeId, id: SignalId, inner: T) -> Self {
        Self {
            scope_id,
            id,
            value: inner,
        }
    }

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
        Scope::add_signal(self);
        &self.value
    }
}

impl<T> fmt::Display for Signal<T>
where
    T: Serialize + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Scope::add_signal(self);
        self.value.fmt(f)
    }
}

impl<T> Deref for SignalMut<T>
where
    T: Serialize,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}

impl<T> DerefMut for SignalMut<T>
where
    T: Serialize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.value
    }
}
