use std::any::Any;
use std::future::Future;
use std::sync::{Arc, Mutex};

use multer::Multipart;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use crate::error::InternalError;

tokio::task_local! {
    static SCOPE: Scope;
    pub static KEY: u32;
}

#[derive(Clone)]
pub struct Scope {
    inner: Arc<Mutex<ScopeBuilder>>,
}

pub(crate) struct ScopeBuilder {
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
    Deserialized(Box<dyn Any + Send>),
}

pub fn event<E>() -> Option<E>
where
    E: DeserializeOwned + Copy + crate::event::Event + Send + 'static,
{
    SCOPE
        .try_with(|scope| {
            let mut state = scope.inner.lock().unwrap();
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
                        Payload::UrlEncoded(payload) if !payload.is_empty() => {
                            match serde_html_form::from_str(payload) {
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
                            }
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(_) => match serde_json::from_str("null")
                            .or_else(|_| serde_json::from_str("{}"))
                        {
                            Ok(payload) => {
                                *event = Event::Deserialized(Box::new(payload));
                                Some(payload)
                            }
                            Err(err) => {
                                state.error = Some(InternalError::Deserialize {
                                    what: "event empty urlencoded payload",
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
            let mut state = scope.inner.lock().unwrap();
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
                        Payload::UrlEncoded(payload) if !payload.is_empty() => {
                            match serde_html_form::from_str(&payload) {
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
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(_) => match serde_json::from_str("null")
                            .or_else(|_| serde_json::from_str("{}"))
                        {
                            Ok(payload) => Some(payload),
                            Err(err) => {
                                state.error = Some(InternalError::Deserialize {
                                    what: "event empty urlencoded payload",
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
            let mut state = scope.inner.lock().unwrap();
            state.multipart.take()
        })
        .ok()
        .flatten()
}

// FIXME: implement builder to avoid locking over and over again
impl Scope {
    pub(crate) fn builder() -> ScopeBuilder {
        ScopeBuilder {
            event: None,
            multipart: None,
            error: None,
        }
    }

    pub async fn run<T>(
        self,
        f: impl Future<Output = Result<T, crate::Error>> + Send,
    ) -> Result<T, crate::Error> {
        let result = SCOPE.scope(self.clone(), f).await;
        let mut inner = self.inner.lock().unwrap();
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
}

impl ScopeBuilder {
    pub(crate) fn with_event(mut self, id: String, payload: Payload) -> Self {
        self.event = Some(Event::Raw { id, payload });
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn with_multipart(mut self, multipart: Multipart<'static>) -> Self {
        self.multipart = Some(multipart);
        self
    }

    pub fn build(self) -> Scope {
        Scope {
            inner: Arc::new(Mutex::new(self)),
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            inner: Arc::new(Mutex::new(Self::builder())),
        }
    }
}
