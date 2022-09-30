use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::Component;
use crate::render::Renderer;
use crate::{IntoView, View, ViewHashTree};

#[linkme::distributed_slice]
pub static COMPONENT_FACTORIES: [fn(&mut ComponentRegistry)] = [..];

type ComponentHandler = dyn Fn(&mut dyn std::io::Read) -> Update + Sync + Send;

pub struct ComponentRegistry {
    handler: HashMap<String, Box<ComponentHandler>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    state: Box<RawValue>,
    hash_tree: ViewHashTree,
    html: String,
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        let mut registry = Self {
            handler: Default::default(),
        };
        for f in COMPONENT_FACTORIES {
            (f)(&mut registry);
        }
        registry
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload<C, M> {
    #[serde(rename = "state")]
    component: C,
    hash_tree: ViewHashTree,
    message: M,
}

impl ComponentRegistry {
    pub fn register<C>(&mut self)
    where
        C: Component,
    {
        self.handler.insert(
            C::id().into_owned(),
            Box::new(move |rd: &mut dyn std::io::Read| {
                // TODO: unwraps
                let Payload::<C, C::Message> {
                    mut component,
                    hash_tree,
                    message,
                } = serde_json::from_reader(rd).unwrap();
                component.update(message);
                let state = serde_json::value::to_raw_value(&component).unwrap();

                let mut r = Renderer::from_previous_tree(hash_tree);
                component.into_view().render(&mut r).unwrap();

                let out = r.end();

                Update {
                    state,
                    hash_tree: out.hash_tree,
                    html: out.view,
                }
            }),
        );
    }

    pub fn handle(&self, id: &str, mut rd: impl std::io::Read + 'static) -> Option<Update> {
        let handler = self.handler.get(id)?;
        Some(handler(&mut rd))
    }
}
