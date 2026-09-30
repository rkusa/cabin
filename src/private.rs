#[cfg(not(target_arch = "wasm32"))]
pub use events::*;
pub use linkme;

/// Registry of all events (added via `#[derive(Event)]`), used to deserialize and validate the
/// payload of an incoming event before handling it (see [`crate::Exposed`]).
#[cfg(not(target_arch = "wasm32"))]
mod events {
    use std::any::Any;
    use std::collections::HashMap;
    use std::future::Future;
    use std::marker::PhantomData;
    use std::pin::Pin;
    use std::sync::OnceLock;

    use http::StatusCode;
    use serde::de::DeserializeOwned;

    use crate::scope::Payload;
    use crate::{Error, Exposed};

    #[linkme::distributed_slice]
    pub static EVENTS: [EventValidator] = [..];

    pub struct EventValidator {
        pub id: &'static str,
        pub validate: ValidateFn,
    }

    /// Returns `None` if the event cannot be deserialized.
    pub type ValidateFn = fn(EventPayload<'_>) -> Option<ValidateFuture>;
    pub type ValidateFuture =
        Pin<Box<dyn Future<Output = Result<Box<dyn Any + Send>, Error>> + Send>>;

    pub struct EventPayload<'a>(pub(crate) &'a Payload);

    pub(crate) fn event_validator(id: &str) -> Option<ValidateFn> {
        static VALIDATORS: OnceLock<HashMap<&'static str, ValidateFn>> = OnceLock::new();
        VALIDATORS
            .get_or_init(|| EVENTS.iter().map(|ev| (ev.id, ev.validate)).collect())
            .get(id)
            .copied()
    }

    /// Autoref specialization to only deserialize and validate events that implement
    /// `DeserializeOwned` (events that aren't deserializable can still be fired, e.g. to trigger
    /// a refresh).
    pub struct Probe<T>(PhantomData<T>);

    impl<T> Probe<T> {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    pub trait ValidateDeserializable {
        fn validate(&self, payload: EventPayload<'_>) -> Option<ValidateFuture>;
    }

    impl<T> ValidateDeserializable for &Probe<T>
    where
        T: DeserializeOwned + Exposed + Send + 'static,
    {
        fn validate(&self, payload: EventPayload<'_>) -> Option<ValidateFuture> {
            let event = crate::scope::deserialize_payload::<T>(payload.0);
            Some(Box::pin(async move {
                let event =
                    event.map_err(|err| Error::from(err).with_status(StatusCode::BAD_REQUEST))?;
                event.validate().await?;
                Ok(Box::new(event) as Box<dyn Any + Send>)
            }))
        }
    }

    pub trait ValidateOther {
        fn validate(&self, payload: EventPayload<'_>) -> Option<ValidateFuture>;
    }

    impl<T> ValidateOther for Probe<T> {
        fn validate(&self, _payload: EventPayload<'_>) -> Option<ValidateFuture> {
            None
        }
    }
}
