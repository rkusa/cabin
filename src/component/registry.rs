use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::action::registry::ActionRegistry;
use crate::html::InputEvent;
use crate::view::hash::ViewHashTree;
use crate::view::HashTree;
use crate::{Render, View};

#[linkme::distributed_slice]
pub static COMPONENT_FACTORIES: [fn(&mut ComponentRegistry)] = [..];

type ComponentHandler = dyn Fn(Box<dyn std::io::Read>, &str, Option<&str>) -> Update + Sync + Send;

pub struct ComponentRegistry {
    handler: HashMap<String, Box<ComponentHandler>>,
    action_registry: Arc<ActionRegistry>,
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
            action_registry: Arc::new(Default::default()),
        };
        for f in COMPONENT_FACTORIES {
            (f)(&mut registry);
        }
        registry
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload<S> {
    state: S,
    hash_tree: ViewHashTree,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EventPayload<S, E> {
    state: S,
    hash_tree: ViewHashTree,
    event: E,
}

impl ComponentRegistry {
    pub fn register<
        S: Serialize + for<'de> Deserialize<'de> + 'static,
        V: View<S>,
        F: Fn(S) -> V + Send + Sync + 'static,
    >(
        &mut self,
        module_path: &str,
        name: &str,
        component: F,
    ) {
        let action_registry = self.action_registry.clone();
        self.handler.insert(
            format!("{}::{}", module_path, name),
            Box::new(
                move |rd: Box<dyn std::io::Read>, action: &str, event: Option<&str>| {
                    if let Some(event) = event {
                        // TODO: unwraps
                        // TODO: better solution than this string based match?
                        let (after, hash_tree) = match event {
                            "input" => {
                                let action = action_registry
                                    .get_event::<S, InputEvent>(action)
                                    .expect("action does not exist");
                                let payload: EventPayload<S, InputEvent> =
                                    serde_json::from_reader(rd).unwrap();
                                (
                                    (action.action)(payload.state, payload.event),
                                    payload.hash_tree,
                                )
                            }
                            _ => panic!("unknown event: {}", event),
                        };

                        let state = serde_json::value::to_raw_value(&after).unwrap();
                        let component = (component)(after);
                        let (html, hash_tree) = render_update(component, hash_tree).unwrap();
                        Update {
                            state,
                            hash_tree,
                            html,
                        }
                    } else {
                        let action = action_registry
                            .get::<S>(action)
                            .expect("action does not exist");

                        // TODO: unwraps
                        let payload: Payload<S> = serde_json::from_reader(rd).unwrap();
                        let after = (action.action)(payload.state);
                        let state = serde_json::value::to_raw_value(&after).unwrap();
                        let component = (component)(after);
                        let (html, hash_tree) =
                            render_update(component, payload.hash_tree).unwrap();
                        Update {
                            state,
                            hash_tree,
                            html,
                        }
                    }
                },
            ),
        );
    }

    pub fn handle(
        &self,
        id: &str,
        rd: impl std::io::Read + 'static,
        action: &str,
    ) -> Option<Update> {
        let handler = self.handler.get(id)?;
        Some(handler(Box::new(rd), action, None))
    }

    pub fn handle_event(
        &self,
        id: &str,
        rd: impl std::io::Read + 'static,
        action: &str,
        event: &str,
    ) -> Option<Update> {
        let handler = self.handler.get(id)?;
        Some(handler(Box::new(rd), action, Some(event)))
    }
}

pub fn render_update<S>(
    view: impl View<S>,
    previous_tree: ViewHashTree,
) -> Result<(String, ViewHashTree), fmt::Error> {
    let mut hash_tree = HashTree::from_previous_tree(previous_tree);
    let renderer = view.render(&mut hash_tree).unwrap(); // TODO: unwrap
    let mut result = String::new();
    renderer.render(&mut result, true)?;
    Ok((result, hash_tree.finish()))
}
