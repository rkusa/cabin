use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use hyper::body::Bytes;
use once_cell::race::OnceBox;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::render::Renderer;
use crate::{View, ViewHashTree};

#[linkme::distributed_slice]
pub static COMPONENT_FACTORIES: [fn(&mut ComponentRegistry)] = [..];

static REGISTRY: OnceBox<ComponentRegistry> = OnceBox::new();

type ComponentHandler =
    dyn Fn(Bytes) -> SyncFuture<Pin<Box<dyn Future<Output = Update> + Send>>> + Send + Sync;

pub struct ComponentRegistry {
    handler: HashMap<(&'static str, &'static str), Arc<ComponentHandler>>,
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

    pub fn register<S, M, V, R, U>(
        &mut self,
        id: &'static str,
        action: &'static str,
        update: fn(S, M) -> U,
        render: fn(S) -> R,
    ) where
        S: Serialize + DeserializeOwned + Send + 'static,
        M: DeserializeOwned + Send + 'static,
        V: View<S> + Send + 'static,
        U: Future<Output = S> + Send + 'static,
        R: Future<Output = V> + Send + 'static,
    {
        self.action_names.insert(update as usize, action);
        self.handler.insert(
            (id, action),
            Arc::new(move |body: Bytes| {
                SyncFuture::new(Box::pin(async move {
                    // TODO: unwraps
                    let Payload::<S, M> {
                        state,
                        hash_tree,
                        payload,
                    } = serde_json::from_slice(&body).unwrap();
                    // TODO: async
                    let state = update(state, payload).await;
                    let state_serialized = serde_json::value::to_raw_value(&state).unwrap();

                    let r = Renderer::from_previous_tree(hash_tree);
                    let view = render(state).await;
                    let fut = view.render(r);
                    let r = fut.await.unwrap();
                    let out = r.end();

                    Update {
                        state: state_serialized,
                        hash_tree: out.hash_tree,
                        html: out.view,
                    }
                }))
            }),
        );
    }

    pub fn action_name(&self, addr: usize) -> Option<&'static str> {
        self.action_names.get(&addr).copied()
    }

    pub async fn handle(&self, id: &str, action: &str, body: Bytes) -> Option<Update> {
        let handler = Arc::clone(self.handler.get(&(id, action))?);
        let fut = handler(body);
        Some(fut.await)
    }
}

// TODO: get rid of?
pub struct SyncFuture<F> {
    inner: F,
}

impl<F: Future> SyncFuture<F> {
    pub fn new(inner: F) -> Self {
        Self { inner }
    }
}
unsafe impl<T> Sync for SyncFuture<T> {}
impl<F: Future + Send> Future for SyncFuture<F> {
    type Output = F::Output;
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let inner = unsafe { self.map_unchecked_mut(|x| &mut x.inner) };
        inner.poll(cx)
    }
}
