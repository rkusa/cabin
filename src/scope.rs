use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::value::RawValue;
use tokio::task::JoinHandle;
use twox_hash::XxHash32;

use crate::error::InternalError;
use crate::state::StateId;

tokio::task_local! {
    static SCOPE: Scope;
    pub static KEY: u32;
}

#[derive(Clone)]
pub struct Scope {
    inner: Rc<RefCell<Inner>>,
}

struct Inner {
    prev_state: Option<HashMap<StateId, Box<RawValue>>>,
    next_state: Vec<u8>,
    event: Option<Event>,
    error: Option<InternalError>,
}

enum Event {
    Raw { id: u32, payload: Box<RawValue> },
    Deserialized(Box<dyn Any>),
}

pub(crate) fn event<E>() -> Option<E>
where
    E: DeserializeOwned + Copy + 'static,
{
    SCOPE
        .try_with(|scope| {
            let mut state = scope.inner.borrow_mut();
            let event = state.event.as_mut()?;
            match event {
                Event::Raw { id, payload } => {
                    let mut hasher = XxHash32::default();
                    TypeId::of::<E>().hash(&mut hasher);
                    let type_id = hasher.finish() as u32;

                    if *id != type_id {
                        return None;
                    }

                    match serde_json::from_str(payload.get()) {
                        Ok(payload) => {
                            *event = Event::Deserialized(Box::new(payload));
                            Some(payload)
                        }
                        Err(err) => {
                            state.error = Some(InternalError::Deserialize {
                                what: "event payload",
                                err,
                            });
                            None
                        }
                    }
                }
                Event::Deserialized(payload) => payload.downcast_ref::<E>().copied(),
            }
        })
        .ok()
        .flatten()
}

pub fn take_event<E>() -> Option<E>
where
    E: DeserializeOwned + 'static,
{
    SCOPE
        .try_with(|scope| {
            let mut state = scope.inner.borrow_mut();
            let event = state.event.take()?;
            match event {
                Event::Raw { id, payload } => {
                    let mut hasher = XxHash32::default();
                    TypeId::of::<E>().hash(&mut hasher);
                    let type_id = hasher.finish() as u32;

                    if id != type_id {
                        state.event = Some(Event::Raw { id, payload });
                        return None;
                    }

                    match serde_json::from_str(payload.get()) {
                        Ok(payload) => Some(payload),
                        Err(err) => {
                            state.error = Some(InternalError::Deserialize {
                                what: "event payload",
                                err,
                            });
                            None
                        }
                    }
                }
                Event::Deserialized(payload) => match payload.downcast::<E>() {
                    Ok(event) => Some(*event),
                    Err(payload) => {
                        state.event = Some(Event::Deserialized(payload));
                        None
                    }
                },
            }
        })
        .ok()
        .flatten()
}

impl Scope {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner {
                prev_state: None,
                next_state: vec![b'{'],
                event: None,
                error: None,
            })),
        }
    }

    pub fn with_prev_state(self, prev_state: HashMap<StateId, Box<RawValue>>) -> Self {
        {
            let mut state = self.inner.borrow_mut();
            state.prev_state = Some(prev_state);
        }
        self
    }

    pub fn with_event(self, id: u32, payload: Box<RawValue>) -> Self {
        {
            let mut state = self.inner.borrow_mut();
            state.event = Some(Event::Raw { id, payload });
        }
        self
    }

    pub async fn run<T>(
        self,
        f: impl Future<Output = Result<T, crate::Error>>,
    ) -> Result<T, crate::Error> {
        let result = SCOPE.scope(self.clone(), f).await;
        let mut inner = self.inner.borrow_mut();
        if let Some(err) = inner.error.take() {
            return Err(err.into());
        }
        result
    }

    pub(crate) fn restore<T>(id: StateId) -> Option<T>
    where
        T: DeserializeOwned,
    {
        SCOPE
            .try_with(|scope| {
                let mut inner = scope.inner.borrow_mut();
                let prev = inner.prev_state.as_mut()?.remove(&id)?;

                match serde_json::from_str(prev.get()) {
                    Ok(payload) => Some(payload),
                    Err(err) => {
                        inner.error = Some(InternalError::Deserialize {
                            what: "previous state",
                            err,
                        });
                        None
                    }
                }
            })
            .ok()
            .flatten()
    }

    pub(crate) fn serialize_state<T>(id: StateId, value: &T)
    where
        T: Serialize,
    {
        SCOPE
            .try_with(|scope| {
                let mut inner = scope.inner.borrow_mut();

                if inner.next_state.len() > 1 {
                    inner.next_state.push(b',');
                }

                let mut ser = serde_json::Serializer::new(&mut inner.next_state);
                if let Err(err) = id.serialize(&mut ser) {
                    inner.error = Some(InternalError::Serialize {
                        what: "state id",
                        err,
                    });
                };
                inner.next_state.push(b':');
                let mut ser = serde_json::Serializer::new(&mut inner.next_state);
                if let Err(err) = value.serialize(&mut ser) {
                    inner.error = Some(InternalError::Serialize { what: "state", err });
                };
            })
            .ok();
    }

    pub fn into_view(self) -> String {
        let mut serialized_state = match Rc::try_unwrap(self.inner) {
            Ok(inner) => inner.into_inner().next_state,
            Err(scope) => {
                let mut inner = scope.borrow_mut();
                std::mem::replace(&mut inner.next_state, vec![b'{'])
            }
        };
        serialized_state.push(b'}');
        String::from_utf8(serialized_state).unwrap()
    }

    pub fn keyed_sync<F, R>(key: u32, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        // let key = hash(key);
        KEY.sync_scope(key, f)
    }

    pub fn keyed<T, F>(key: u32, f: F) -> impl Future<Output = T>
    where
        F: Future<Output = T>,
    {
        // let key = hash(key);
        KEY.scope(key, f)
    }

    pub fn key() -> Option<u32> {
        KEY.try_with(|key| *key).ok()
    }

    pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        let scope = SCOPE
            .try_with(|scope| scope.clone())
            .expect("not called within scope");
        tokio::task::spawn_local(SCOPE.scope(scope, future))
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
