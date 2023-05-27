use serde::Serialize;

use crate::html::attributes::Attributes;
use crate::html::events::InputEvent;
use crate::html::Html;
use crate::render::ElementRenderer;
use crate::View;

pub struct Input<Ev> {
    on_input: Option<Ev>,
}

impl<V, Ev, A> Html<V, Ev, A, Input<Ev>>
where
    V: View<Ev>,
{
    pub fn on_input(mut self, event: impl FnOnce(InputEvent) -> Ev) -> Self {
        self.kind.on_input = Some(event(InputEvent::default()));
        self
    }
}

impl<Ev> Attributes for Input<Ev>
where
    Ev: Serialize,
{
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

impl<Ev> Default for Input<Ev> {
    fn default() -> Self {
        Self { on_input: None }
    }
}
