use std::any::TypeId;
use std::hash::{Hash, Hasher};

use serde::Serialize;
use twox_hash::XxHash32;

use crate::html::events::InputEvent;
use crate::html::{ElementExt, Html};
use crate::render::ElementRenderer;
use crate::View;

#[derive(Default)]
pub struct Input {
    // FIXME: no box?
    on_input: Option<Box<dyn FnOnce() -> (u32, String)>>,
}

impl<V> Html<V, Input>
where
    V: View,
{
    pub fn on_input<E>(mut self, event: impl FnOnce(InputEvent) -> E) -> Self
    where
        E: Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.kind.on_input = Some(Box::new(move || {
            let mut hasher = XxHash32::default();
            TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;

            // FIXME: unwrap
            (hash, serde_json::to_string(&event).unwrap())
        }));

        self
    }
}

impl ElementExt for Input {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        if let Some(event) = self.on_input {
            // FIXME: directly write into r?
            let (id, payload) = &(event)();
            r.attribute("cabin-input", id)
                .map_err(crate::error::InternalError::from)?;
            r.attribute("cabin-input-payload", payload)
                .map_err(crate::error::InternalError::from)?;
        }

        Ok(())
    }
}
