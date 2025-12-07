use std::any::Any;
use std::cell::RefCell;

use multer::Multipart;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use crate::error::InternalError;
use crate::render::Renderer;

tokio::task_local! {
    static CONTEXT: Context;
}

pub struct Context {
    renderer_pool: RefCell<Vec<Renderer>>,
    event: RefCell<Option<Event>>,
    multipart: RefCell<Option<Multipart<'static>>>,
    error: RefCell<Option<InternalError>>,
    is_update: bool,
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
            event: RefCell::new(None),
            multipart: RefCell::new(None),
            error: RefCell::new(None),
            is_update,
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

    pub(crate) fn acquire_renderer(&self) -> Renderer {
        self.renderer_pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Renderer::new(self.is_update))
    }

    pub(crate) fn release_renderer(&self, mut renderer: Renderer) {
        renderer.reset();
        self.renderer_pool.borrow_mut().push(renderer);
    }

    pub(crate) fn acquire_renderer_from_task() -> Renderer {
        let r = CONTEXT.try_with(|c| c.acquire_renderer()).ok();
        debug_assert!(r.is_some());

        r.unwrap_or_else(|| Renderer::new(false))
    }

    pub(crate) fn release_renderer_into_task(r: Renderer) {
        let _ = CONTEXT.try_with(|c| {
            c.release_renderer(r);
        });
    }

    pub async fn run<T>(
        self,
        f: impl Future<Output = Result<T, crate::Error>>,
    ) -> Result<T, crate::Error> {
        CONTEXT
            .scope(self, async {
                let t = f.await?;
                CONTEXT.with(|c| {
                    if let Some(err) = c.error.borrow_mut().take() {
                        return Err(err);
                    }
                    Ok(())
                })?;
                Ok(t)
            })
            .await
    }
}

pub fn event<E>() -> Option<E>
where
    E: DeserializeOwned + Copy + crate::event::Event + 'static,
{
    CONTEXT
        .try_with(|context| {
            let mut event = context.event.borrow_mut();
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
                                (*context.error.borrow_mut()) = Some(InternalError::Deserialize {
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
                                    (*context.error.borrow_mut()) =
                                        Some(InternalError::Deserialize {
                                            what: "event urlencoded payload".into(),
                                            err: Box::new(err),
                                        });
                                    None
                                }
                            }
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(payload) => {
                            match serde_json::from_str("null")
                                .or_else(|_| serde_json::from_str("{}"))
                            {
                                Ok(payload) => {
                                    *event = Event::Deserialized(Box::new(payload));
                                    Some(payload)
                                }
                                Err(err) => {
                                    tracing::debug!(payload, "event payload");
                                    (*context.error.borrow_mut()) =
                                        Some(InternalError::Deserialize {
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
        })
        .ok()
        .flatten()
}

pub fn take_event<E>() -> Option<E>
where
    E: DeserializeOwned + crate::event::Event + 'static,
{
    CONTEXT
        .try_with(|context| {
            let mut event = context.event.borrow_mut();
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
                                (*context.error.borrow_mut()) = Some(InternalError::Deserialize {
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
                                    (*context.error.borrow_mut()) =
                                        Some(InternalError::Deserialize {
                                            what: "event urlencoded payload".into(),
                                            err: Box::new(err),
                                        });
                                    None
                                }
                            }
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        Payload::UrlEncoded(payload) => {
                            match serde_json::from_str("null")
                                .or_else(|_| serde_json::from_str("{}"))
                            {
                                Ok(payload) => Some(payload),
                                Err(err) => {
                                    tracing::debug!(payload, "event payload");
                                    (*context.error.borrow_mut()) =
                                        Some(InternalError::Deserialize {
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
        })
        .ok()
        .flatten()
}

pub fn take_multipart() -> Option<Multipart<'static>> {
    CONTEXT
        .try_with(|context| context.multipart.borrow_mut().take())
        .ok()
        .flatten()
}

pub fn is_update() -> bool {
    CONTEXT.try_with(|c| c.is_update()).ok().unwrap_or(false)
}

impl Drop for Context {
    fn drop(&mut self) {
        eprintln!("poll size: {}", self.renderer_pool.borrow().len());
    }
}
