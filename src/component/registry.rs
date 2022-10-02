use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use hyper::body::Bytes;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use super::Component;
use crate::render::Renderer;
use crate::{IntoView, Render, View, ViewHashTree};

#[linkme::distributed_slice]
pub static COMPONENT_FACTORIES: [fn(&mut ComponentRegistry)] = [..];

type ComponentHandler =
    dyn Fn(Bytes) -> SyncFuture<Pin<Box<dyn Future<Output = Update> + Send>>> + Send + Sync;

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
        C: Component + Send + 'static,
        for<'v> <C as Render>::View<'v>: Send,
        for<'v> C::Message<'v>: Send,
    {
        self.handler.insert(
            C::id().into_owned(),
            Arc::new(move |body: Bytes| {
                SyncFuture::new(Box::pin(async move {
                    // TODO: unwraps
                    let Payload::<C, C::Message<'_>> {
                        mut component,
                        hash_tree,
                        message,
                    } = serde_json::from_slice(&body).unwrap();
                    component.update(message).await;
                    let state = serde_json::value::to_raw_value(&component).unwrap();

                    let r = Renderer::from_previous_tree(hash_tree);
                    let view = component.into_view();
                    let fut = view.render(r);
                    let r = fut.await.unwrap();

                    let out = r.end();

                    Update {
                        state,
                        hash_tree: out.hash_tree,
                        html: out.view,
                    }
                }))
            }),
        );
    }

    pub async fn handle(&self, id: &str, body: Bytes) -> Option<Update> {
        let handler = Arc::clone(self.handler.get(id)?);
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
