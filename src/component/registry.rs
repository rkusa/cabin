use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use once_cell::race::OnceBox;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::Component;
use crate::render::{PreviousComponent, Renderer};
use crate::restore::PREVIOUS;
use crate::{View, ViewHashTree};

#[linkme::distributed_slice]
pub static COMPONENT_FACTORIES: [fn(&mut ComponentRegistry)] = [..];

static REGISTRY: OnceBox<ComponentRegistry> = OnceBox::new();

type ComponentFuture = Pin<Box<dyn Future<Output = Result<Update, crate::Error>>>>;
type ComponentHandler = dyn Fn(Bytes) -> ComponentFuture + Send + Sync;

pub struct ComponentRegistry {
    handler: HashMap<String, Arc<ComponentHandler>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    state: Box<RawValue>,
    hash_tree: ViewHashTree,
    html: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload<C>
where
    C: Component,
{
    #[serde(rename = "state")]
    component: C,
    hash_tree: ViewHashTree,
    event: C::Event,
    descendants: HashMap<u32, PreviousComponent>,
}

impl ComponentRegistry {
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let mut registry = Self {
                handler: Default::default(),
            };
            for f in COMPONENT_FACTORIES {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    pub fn register<C>(&mut self)
    where
        C: Component + Serialize + DeserializeOwned,
    {
        self.handler.insert(
            C::id().to_string(),
            Arc::new(move |body: Bytes| {
                Box::pin(async move {
                    // TODO: unwraps
                    let Payload::<C> {
                        mut component,
                        hash_tree,
                        event,
                        descendants,
                    } = serde_json::from_slice(&body).unwrap();
                    component.update(event).await;
                    let state_serialized = serde_json::value::to_raw_value(&component).unwrap();

                    let r = Renderer::from_previous_tree(hash_tree);
                    let r = PREVIOUS
                        .scope(RefCell::new(Some(descendants)), async {
                            let view = component.view().await.map_err(|err| err.into())?;
                            view.render(r).await
                        })
                        .await?;
                    let out = r.end();

                    Ok(Update {
                        state: state_serialized,
                        hash_tree: out.hash_tree,
                        html: out.view,
                    })
                })
            }),
        );
    }

    pub async fn handle(&self, id: &str, body: Bytes) -> Result<Option<Update>, crate::Error> {
        let Some(handler) = self.handler.get(id) else {
            return Ok(None)
        };
        let handler = Arc::clone(handler);
        crate::local_pool::spawn(move || handler(body))
            .await
            .map(Some)
    }
}
