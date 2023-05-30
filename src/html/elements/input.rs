use serde::Serialize;

use crate::html::attributes::Attributes;
use crate::html::events::InputEvent;
use crate::html::Html;
use crate::render::ElementRenderer;
use crate::View;

pub struct Input {
    on_input: Option,
}

impl<V, A> Html<V, A, Input>
where
    V: View,
{
    pub fn on_input(mut self, event: impl FnOnce(InputEvent) -> Ev) -> Self {
        self.kind.on_input = Some(event(InputEvent::default()));
        self
    }
}

impl Attributes for Input {
    fn render(&self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        if let Some(event) = &self.on_input {
            // TODO: unwrap
            let event = serde_json::to_string(&event).unwrap();
            r.attribute("data-input", &event)
                .map_err(crate::error::InternalError::from)?;
        }

        Ok(())
    }
}

impl Default for Input {
    fn default() -> Self {
        Self { on_input: None }
    }
}
