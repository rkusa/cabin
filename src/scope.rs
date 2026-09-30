use std::any::Any;
use std::cell::RefCell;
use std::future::Future;

use multer::Multipart;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

use crate::error::InternalError;
use crate::render::Renderer;

tokio::task_local! {
    static SCOPE: Scope;
}

pub struct Scope {
    event: RefCell<Option<Event>>,
    multipart: RefCell<Option<Multipart<'static>>>,
    error: RefCell<Option<InternalError>>,
    renderer_pool: RefCell<Vec<Renderer>>,
    is_update: bool,
    disable_hashes: bool,
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
                    if *id != E::ID {
                        return None;
                    }

                    match deserialize_payload::<E>(payload) {
                        Ok(payload) => {
                            *event = Event::Deserialized(Box::new(payload));
                            Some(payload)
                        }
                        Err(err) => {
                            (*scope.error.borrow_mut()) = Some(err);
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
    E: DeserializeOwned + crate::event::Event + 'static,
{
    SCOPE
        .try_with(|scope| {
            let mut event = scope.event.borrow_mut();
            match event.take()? {
                Event::Raw { id, payload } => {
                    if id != E::ID {
                        *event = Some(Event::Raw { id, payload });
                        return None;
                    }

                    match deserialize_payload::<E>(&payload) {
                        Ok(payload) => Some(payload),
                        Err(err) => {
                            (*scope.error.borrow_mut()) = Some(err);
                            None
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

pub(crate) fn deserialize_payload<E: DeserializeOwned>(
    payload: &Payload,
) -> Result<E, InternalError> {
    match payload {
        Payload::Json(payload) => serde_json::from_str(payload.get()).map_err(|err| {
            tracing::debug!(?payload, "event payload");
            InternalError::Deserialize {
                what: "event json payload",
                err: Box::new(err),
            }
        }),
        #[cfg(not(target_arch = "wasm32"))]
        Payload::UrlEncoded(payload) if !payload.is_empty() => serde_html_form::from_str(payload)
            .map_err(|err| {
                tracing::debug!(payload, "event payload");
                InternalError::Deserialize {
                    what: "event urlencoded payload",
                    err: Box::new(err),
                }
            }),
        #[cfg(not(target_arch = "wasm32"))]
        Payload::UrlEncoded(payload) => serde_json::from_str("null")
            .or_else(|_| serde_json::from_str("{}"))
            .map_err(|err| {
                tracing::debug!(payload, "event payload");
                InternalError::Deserialize {
                    what: "event empty urlencoded payload",
                    err: Box::new(err),
                }
            }),
    }
}

pub fn take_multipart() -> Option<Multipart<'static>> {
    SCOPE
        .try_with(|scope| scope.multipart.borrow_mut().take())
        .ok()
        .flatten()
}

impl Scope {
    pub(crate) fn new(is_update: bool, disable_hashes: bool) -> Self {
        Self {
            event: Default::default(),
            multipart: Default::default(),
            error: Default::default(),
            renderer_pool: Default::default(),
            is_update,
            disable_hashes,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn with_event(self, id: String, payload: Payload) -> Self {
        *(self.event.borrow_mut()) = Some(Event::Raw { id, payload });
        self
    }

    /// Like [`Scope::with_event`], but if the event type is known (see
    /// [`crate::private::EVENTS`]), deserializes the payload and validates it (see
    /// [`crate::Exposed`]) right away.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) async fn with_validated_event(
        self,
        id: String,
        payload: Payload,
    ) -> Result<Self, crate::Error> {
        let validate = crate::private::event_validator(&id)
            .and_then(|validate| validate(crate::private::EventPayload(&payload)));
        let event = match validate {
            Some(validate) => Event::Deserialized(validate.await?),
            None => Event::Raw { id, payload },
        };
        *(self.event.borrow_mut()) = Some(event);
        Ok(self)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn with_multipart(self, multipart: Multipart<'static>) -> Self {
        *(self.multipart.borrow_mut()) = Some(multipart);
        self
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }

    pub(crate) fn create_renderer(&self) -> Renderer {
        self.renderer_pool
            .borrow_mut()
            .pop()
            .unwrap_or_else(|| Renderer::new(self.is_update, self.disable_hashes))
    }

    pub(crate) fn create_renderer_from_task() -> Renderer {
        SCOPE
            .try_with(|c| c.create_renderer())
            .unwrap_or_else(|_| Renderer::new(false, false))
    }

    pub(crate) fn release_renderer(&self, mut r: Renderer) {
        r.reset();
        self.renderer_pool.borrow_mut().push(r);
    }

    pub(crate) fn release_renderer_to_task(r: Renderer) {
        SCOPE.try_with(|c| c.release_renderer(r)).ok();
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
