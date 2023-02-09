use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use once_cell::race::OnceBox;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::id::NanoId;
use super::ComponentId;
use crate::render::{PreviousComponent, Renderer};
use crate::{View, ViewHashTree};

#[linkme::distributed_slice]
pub static COMPONENT_FACTORIES: [fn(&mut ComponentRegistry)] = [..];

static REGISTRY: OnceBox<ComponentRegistry> = OnceBox::new();

type ComponentHandler = dyn Fn(Bytes) -> Pin<Box<dyn Future<Output = Result<Update, crate::Error>> + Send>>
    + Send
    + Sync;

pub struct ComponentRegistry {
    handler: HashMap<(Cow<'static, str>, &'static str), Arc<ComponentHandler>>,
    action_names: HashMap<usize, &'static str>,
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
struct Payload<S, M> {
    state: S,
    hash_tree: ViewHashTree,
    payload: M,
    descendants: HashMap<NanoId, PreviousComponent>,
}

impl ComponentRegistry {
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let mut registry = Self {
                handler: Default::default(),
                action_names: Default::default(),
            };
            for f in COMPONENT_FACTORIES {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    pub fn register<S, M, V, R, U, E>(
        &mut self,
        id: ComponentId,
        action: &'static str,
        update: fn(S, M) -> U,
        render: fn(S) -> R,
    ) where
        S: Serialize + DeserializeOwned + Send + 'static,
        M: DeserializeOwned + Send + 'static,
        V: View + Send + 'static,
        E: Send + 'static,
        crate::Error: From<E>,
        U: Future<Output = S> + Send + 'static,
        R: Future<Output = Result<V, E>> + Send + 'static,
    {
        self.action_names.insert(update as usize, action);
        self.handler.insert(
            (id.to_string().into(), action),
            Arc::new(move |body: Bytes| {
                Box::pin(async move {
                    // TODO: unwraps
                    let Payload::<S, M> {
                        state,
                        hash_tree,
                        payload,
                        descendants,
                    } = serde_json::from_slice(&body).unwrap();
                    // TODO: async
                    let state = update(state, payload).await;
                    let state_serialized = serde_json::value::to_raw_value(&state).unwrap();

                    let r = Renderer::from_previous_tree(hash_tree).with_descendants(descendants);
                    let view = render(state).await?;
                    let fut = view.render(r);
                    let r = fut.await.unwrap();
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

    pub fn action_name(&self, addr: usize) -> Option<&'static str> {
        self.action_names.get(&addr).copied()
    }

    pub async fn handle(
        &self,
        id: &str,
        action: &str,
        body: Bytes,
    ) -> Result<Option<Update>, crate::Error> {
        let Some(handler) = self.handler.get(&(id.into(), action)) else {
            return Ok(None)
        };
        let handler = Arc::clone(handler);
        let fut = handler(body);
        fut.await.map(Some)
    }
}
