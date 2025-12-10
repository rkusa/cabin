use std::any::Any;
use std::cell::RefCell;
use std::future::Future;

use multer::Multipart;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use crate::error::InternalError;

tokio::task_local! {
    static SCOPE: Scope;
}

pub struct Scope {
    event: RefCell<Option<Event>>,
    multipart: RefCell<Option<Multipart<'static>>>,
    error: RefCell<Option<InternalError>>,
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
            let mut event = scope.event.borrow_mut();
            let event = event.as_mut()?;
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
                                tracing::debug!(?payload, "event payload");
                                (*scope.error.borrow_mut()) = Some(InternalError::Deserialize {
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
                                    tracing::debug!(payload, "event payload");
                                    (*scope.error.borrow_mut()) =
                                        Some(InternalError::Deserialize {
                                            what: "event urlencoded payload",
                                            err: Box::new(err),
                                        });
                                    None
                                }
                            }
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(payload) => match serde_json::from_str("null")
                            .or_else(|_| serde_json::from_str("{}"))
                        {
                            Ok(payload) => {
                                *event = Event::Deserialized(Box::new(payload));
                                Some(payload)
                            }
                            Err(err) => {
                                tracing::debug!(payload, "event payload");
                                (*scope.error.borrow_mut()) = Some(InternalError::Deserialize {
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
            let mut event = scope.event.borrow_mut();
            match event.take()? {
                Event::Raw { id, payload } => {
                    let event_id = E::ID;
                    if id != event_id {
                        *event = Some(Event::Raw { id, payload });
                        return None;
                    }

                    match payload {
                        Payload::Json(payload) => match serde_json::from_str(payload.get()) {
                            Ok(payload) => Some(payload),
                            Err(err) => {
                                tracing::debug!(?payload, "event payload");
                                (*scope.error.borrow_mut()) = Some(InternalError::Deserialize {
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
                                    tracing::debug!(payload, "event payload");
                                    (*scope.error.borrow_mut()) =
                                        Some(InternalError::Deserialize {
                                            what: "event urlencoded payload",
                                            err: Box::new(err),
                                        });
                                    None
                                }
                            }
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(payload) => match serde_json::from_str("null")
                            .or_else(|_| serde_json::from_str("{}"))
                        {
                            Ok(payload) => Some(payload),
                            Err(err) => {
                                tracing::debug!(payload, "event payload");
                                (*scope.error.borrow_mut()) = Some(InternalError::Deserialize {
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
                        *event = Some(Event::Deserialized(payload));
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
        .try_with(|scope| scope.multipart.borrow_mut().take())
        .ok()
        .flatten()
}

// FIXME: implement builder to avoid locking over and over again
impl Scope {
    pub(crate) fn new() -> Self {
        Self {
            event: RefCell::new(None),
            multipart: RefCell::new(None),
            error: RefCell::new(None),
        }
    }

    pub(crate) fn with_event(self, id: String, payload: Payload) -> Self {
        *(self.event.borrow_mut()) = Some(Event::Raw { id, payload });
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn with_multipart(self, multipart: Multipart<'static>) -> Self {
        *(self.multipart.borrow_mut()) = Some(multipart);
        self
    }

    pub async fn run<T>(
        self,
        f: impl Future<Output = Result<T, crate::Error>> + Send,
    ) -> Result<T, crate::Error> {
        SCOPE
            .scope(self, async {
                let t = f.await?;
                SCOPE.with(|s| {
                    if let Some(err) = s.error.borrow_mut().take() {
                        return Err(err);
                    }
                    Ok(())
                })?;
                Ok(t)
            })
            .await
    }
}
