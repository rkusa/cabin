use std::future::Future;

use crate::component::registry::ComponentRegistry;
use crate::html::attributes::Attributes;
use crate::html::events::InputEvent;
use crate::html::Html;
use crate::render::ElementRenderer;

#[derive(Default)]
pub struct Input {
    on_input: Option<&'static str>,
}

impl<V, A> Html<V, A, Input> {
    pub fn on_input<M, F: Future<Output = M>>(mut self, action: fn(M, InputEvent) -> F) -> Self {
        let name = ComponentRegistry::global().action_name(action as usize);
        debug_assert!(name.is_some(), "action not registered");
        self.kind.on_input = name;
        self
    }
}

impl Attributes for Input {
    fn render(&self, r: &mut ElementRenderer) -> Result<(), std::fmt::Error> {
        if let Some(on_input) = &self.on_input {
            r.attribute("data-input", on_input)?;
        }

        Ok(())
    }
}
