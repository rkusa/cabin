use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde_json::value::RawValue;
use tokio::task::JoinHandle;
use twox_hash::XxHash32;

use crate::error::InternalError;

tokio::task_local! {
    static SCOPE: Scope;
    pub static KEY: u32;
}

#[derive(Clone)]
pub struct Scope {
    inner: Rc<RefCell<Inner>>,
}

struct Inner {
    event: Option<Event>,
    error: Option<InternalError>,
}

pub(crate) enum Payload {
    Json(Box<RawValue>),
    UrlEncoded(String),
}

enum Event {
    Raw { id: u32, payload: Payload },
    Deserialized(Box<dyn Any>),
}

pub fn event<E>() -> Option<E>
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

                    match payload {
                        Payload::Json(payload) => match serde_json::from_str(payload.get()) {
                            Ok(payload) => {
                                *event = Event::Deserialized(Box::new(payload));
                                Some(payload)
                            }
                            Err(err) => {
                                state.error = Some(InternalError::Deserialize {
                                    what: "event json payload",
                                    err: Box::new(err),
                                });
                                None
                            }
                        },
                        Payload::UrlEncoded(payload) => match serde_urlencoded::from_str(payload) {
                            Ok(payload) => {
                                *event = Event::Deserialized(Box::new(payload));
                                Some(payload)
                            }
                            Err(err) => {
                                state.error = Some(InternalError::Deserialize {
                                    what: "event urlencoded payload",
                                    err: Box::new(err),
                                });
                                None
                            }
                        },
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

                    match payload {
                        Payload::Json(payload) => match serde_json::from_str(payload.get()) {
                            Ok(payload) => Some(payload),
                            Err(err) => {
                                state.error = Some(InternalError::Deserialize {
                                    what: "event json payload",
                                    err: Box::new(err),
                                });
                                None
                            }
                        },
                        Payload::UrlEncoded(payload) => {
                            match serde_urlencoded::from_str(&payload) {
                                Ok(payload) => Some(payload),
                                Err(err) => {
                                    state.error = Some(InternalError::Deserialize {
                                        what: "event urlencoded payload",
                                        err: Box::new(err),
                                    });
                                    None
                                }
                            }
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
                event: None,
                error: None,
            })),
        }
    }

    pub(crate) fn with_event(self, id: u32, payload: Payload) -> Self {
        {
            let mut state = self.inner.borrow_mut();
            state.event = Some(Event::Raw { id, payload });
        }
        self
    }

    pub(crate) fn with_deserialized_event(self, event: Box<dyn Any>) -> Self {
        {
            let mut state = self.inner.borrow_mut();
            state.event = Some(Event::Deserialized(event));
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
