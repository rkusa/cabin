use std::any::Any;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use multer::Multipart;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;
use tokio::task::JoinHandle;
use tracing::Instrument;

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
    multipart: Option<Multipart<'static>>,
    error: Option<InternalError>,
}

pub(crate) enum Payload {
    Json(Box<RawValue>),
    #[cfg(not(target_arch = "wasm32"))]
    UrlEncoded(String),
}

enum Event {
    Raw { id: String, payload: Payload },
    Deserialized(Box<dyn Any>),
}

pub fn event<E>() -> Option<E>
where
    E: DeserializeOwned + Copy + crate::event::Event + 'static,
{
    SCOPE
        .try_with(|scope| {
            let mut state = scope.inner.borrow_mut();
            let event = state.event.as_mut()?;
            match event {
                Event::Raw { id, payload } => {
                    let event_id = E::ID;
                    if *id != event_id {
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
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(payload) => match serde_html_form::from_str(payload) {
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
    E: DeserializeOwned + crate::event::Event + 'static,
{
    SCOPE
        .try_with(|scope| {
            let mut state = scope.inner.borrow_mut();
            let event = state.event.take()?;
            match event {
                Event::Raw { id, payload } => {
                    let event_id = E::ID;
                    if id != event_id {
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
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(payload) => match serde_html_form::from_str(&payload) {
                            Ok(payload) => Some(payload),
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

pub fn take_multipart() -> Option<Multipart<'static>> {
    SCOPE
        .try_with(|scope| {
            let mut state = scope.inner.borrow_mut();
            state.multipart.take()
        })
        .ok()
        .flatten()
}

impl Scope {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner {
                event: None,
                multipart: None,
                error: None,
            })),
        }
    }

    pub(crate) fn with_event(self, id: String, payload: Payload) -> Self {
        {
            let mut state = self.inner.borrow_mut();
            state.event = Some(Event::Raw { id, payload });
        }
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn with_multipart(self, multipart: Multipart<'static>) -> Self {
        {
            let mut state = self.inner.borrow_mut();
            state.multipart = Some(multipart);
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
        let span = tracing::trace_span!("spawn_local_scope");
        tokio::task::spawn_local(SCOPE.scope(scope, future.instrument(span)))
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
