pub mod registry;

use std::fmt::{self, Write};
use std::future::Future;
use std::marker::PhantomData;

use serde::Serialize;
use serde_json::value::RawValue;

use crate::render::Renderer;
use crate::view::View;
use crate::ViewHashTree;

pub struct ServerComponent<F, V, S> {
    id: &'static str,
    state: S,
    component: fn(S) -> F,
    marker: PhantomData<V>,
}

impl<F, V, S> ServerComponent<F, V, S> {
    pub fn new(id: &'static str, state: S, component: fn(S) -> F) -> Self {
        Self {
            id,
            state,
            component,
            marker: PhantomData,
        }
    }
}

impl<F, V, S> View<()> for ServerComponent<F, V, S>
where
    F: Future<Output = V> + Send,
    V: View<S> + Send + 'static,
    S: Serialize + Send,
{
    type Future = impl Future<Output = Result<Renderer, fmt::Error>> + Send;

    fn render(self, mut r: Renderer) -> Self::Future {
        async move {
            if r.is_update() {
                let view = (self.component)(self.state).await;
                return view.render(r).await;
            }

            // TODO: unwrap
            // TODO: Box (of raw value)
            let state_serialized = serde_json::value::to_raw_value(&self.state).unwrap();

            let content_renderer = Renderer::new();
            let view = (self.component)(self.state).await;
            let content_renderer = view.render(content_renderer).await?;
            let out = content_renderer.end();

            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Initial<'a> {
                state: Box<RawValue>,
                hash_tree: &'a ViewHashTree,
            }
            let initial = serde_json::to_string(&Initial {
                state: state_serialized,
                hash_tree: &out.hash_tree,
            })
            .unwrap();

            write!(
                r,
                r#"<server-component data-id="{}"><script type="application/json">{}</script>{}</server-component>"#,
                self.id, initial, out.view
            )?;

            Ok(r)
        }
    }
}
