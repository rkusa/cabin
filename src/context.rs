use std::any::Any;
use std::cell::RefCell;

use multer::Multipart;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use crate::error::InternalError;
use crate::fragment::Fragment;
use crate::render::Renderer;

pub struct Context {
    renderer_pool: RefCell<Vec<Renderer>>,
    is_update: bool,
    event: RefCell<Option<Event>>,
    multipart: RefCell<Option<Multipart<'static>>>,
    error: RefCell<Option<InternalError>>,
}

enum Event {
    Raw { id: String, payload: Payload },
    Deserialized(Box<dyn Any>),
}

pub(crate) enum Payload {
    Json(Box<RawValue>),
    #[cfg(not(target_arch = "wasm32"))]
    UrlEncoded(String),
}

impl Context {
    pub fn new(is_update: bool) -> Self {
        Self {
            renderer_pool: RefCell::new(Vec::with_capacity(8)),
            is_update,
            event: RefCell::new(None),
            multipart: RefCell::new(None),
            error: RefCell::new(None),
        }
    }

    pub(crate) fn with_event(self, id: String, payload: Payload) -> Self {
        (*self.event.borrow_mut()) = Some(Event::Raw { id, payload });
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn with_multipart(self, multipart: Multipart<'static>) -> Self {
        (*self.multipart.borrow_mut()) = Some(multipart);
        self
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }

    pub fn fragment(&self) -> Fragment<'_> {
        Fragment::new(self.acquire_renderer(), self)
    }

    pub(crate) fn acquire_renderer(&self) -> Renderer {
        let mut pool = self.renderer_pool.borrow_mut();
        pool.pop().unwrap_or_else(|| Renderer::new(self.is_update))
    }

    pub(crate) fn release_renderer(&self, mut renderer: Renderer) {
        renderer.reset();
        let mut pool = self.renderer_pool.borrow_mut();
        pool.push(renderer);
    }

    pub fn event<E>(&self) -> Option<E>
    where
        E: DeserializeOwned + Copy + crate::event::Event + 'static,
    {
        let mut event = self.event.borrow_mut();
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
                            (*self.error.borrow_mut()) = Some(InternalError::Deserialize {
                                what: "event json payload".into(),
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
                                (*self.error.borrow_mut()) = Some(InternalError::Deserialize {
                                    what: "event urlencoded payload".into(),
                                    err: Box::new(err),
                                });
                                None
                            }
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    Payload::UrlEncoded(payload) => {
                        match serde_json::from_str("null").or_else(|_| serde_json::from_str("{}")) {
                            Ok(payload) => {
                                *event = Event::Deserialized(Box::new(payload));
                                Some(payload)
                            }
                            Err(err) => {
                                tracing::debug!(payload, "event payload");
                                (*self.error.borrow_mut()) = Some(InternalError::Deserialize {
                                    what: "event empty urlencoded payload".into(),
                                    err: Box::new(err),
                                });
                                None
                            }
                        }
                    }
                }
            }
            Event::Deserialized(payload) => payload.downcast_ref::<E>().copied(),
        }
    }

    pub fn take_event<E>(&self) -> Option<E>
    where
        E: DeserializeOwned + crate::event::Event + 'static,
    {
        let mut event = self.event.borrow_mut();
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
                            (*self.error.borrow_mut()) = Some(InternalError::Deserialize {
                                what: "event json payload".into(),
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
                                (*self.error.borrow_mut()) = Some(InternalError::Deserialize {
                                    what: "event urlencoded payload".into(),
                                    err: Box::new(err),
                                });
                                None
                            }
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    Payload::UrlEncoded(payload) => {
                        match serde_json::from_str("null").or_else(|_| serde_json::from_str("{}")) {
                            Ok(payload) => Some(payload),
                            Err(err) => {
                                tracing::debug!(payload, "event payload");
                                (*self.error.borrow_mut()) = Some(InternalError::Deserialize {
                                    what: "event empty urlencoded payload".into(),
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
                    *event = Some(Event::Deserialized(payload));
                    None
                }
            },
        }
    }

    pub fn take_multipart(&self) -> Option<Multipart<'static>> {
        self.multipart.take()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        eprintln!("poll size: {}", self.renderer_pool.borrow().len());
    }
}
