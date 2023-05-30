use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use once_cell::race::OnceBox;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::signal::{ScopeId, SignalId, SignalMut};

#[linkme::distributed_slice]
pub static ACTION_FACTORIES: [fn(&mut ActionsRegistry)] = [..];

static REGISTRY: OnceBox<ActionsRegistry> = OnceBox::new();

pub struct ActionsRegistry {
    action_names: HashMap<usize, &'static str>,
}

impl ActionsRegistry {
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let mut registry = Self {
                action_names: Default::default(),
            };
            for f in ACTION_FACTORIES {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    // TODO: unique name/id
    pub fn register<T>(&mut self, name: &'static str, action: fn(SignalMut<T>))
    where
        T: Serialize,
    {
        self.action_names.insert(action as usize, name);
    }

    pub fn register_dependency(&mut self, scope_id: ScopeId, signal_id: SignalId) {
        //
    }

    pub fn action_name(&self, addr: usize) -> Option<&'static str> {
        self.action_names.get(&addr).copied()
    }
}
