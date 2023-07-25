use cabin_macros::element;

use super::common::Common;
use super::global::Global;
use super::input::OnChange;
use crate::error::InternalError;
use crate::html::events::InputEvent;
use crate::html::Aria;

// TODO:
#[element]
pub trait Select: Common + Global + Aria {
    fn on_change<E>(self, event: impl FnOnce(InputEvent) -> E) -> impl Select
    where
        E: ::serde::Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.with(OnChange(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_change event",
                    err,
                })
                .map(|json| (hash, json))
        })))
    }
}
