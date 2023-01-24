pub mod id;
pub mod registry;

use std::fmt::{self, Write};
use std::future::Future;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::pin::Pin;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::value::RawValue;

use crate::previous::FromPrevious;
use crate::render::Renderer;
use crate::view::View;
use crate::ViewHashTree;

pub struct ServerComponent<F, V, P, S> {
    id: ComponentId,
    state: P,
    component: fn(S) -> F,
    marker: PhantomData<(V, P)>,
}

#[derive(Clone, Copy, Hash)]
pub struct ComponentId {
    module: &'static str,
    name: &'static str,
}

impl<F, V, P, S> ServerComponent<F, V, P, S> {
    pub fn new(id: ComponentId, state: P, component: fn(S) -> F) -> Self {
        Self {
            id,
            state,
            component,
            marker: PhantomData,
        }
    }
}

impl ComponentId {
    pub const fn new(module: &'static str, name: &'static str) -> Self {
        Self { module, name }
    }
}

impl<F, V, P, S> View for ServerComponent<F, V, P, S>
where
    F: Future<Output = V> + Send + 'static,
    V: View + Send + 'static,
    P: FromPrevious<S> + 'static,
    S: Default + Serialize + DeserializeOwned + Send + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, mut r: Renderer) -> Self::Future {
        Box::pin(async move {
            let (id, prev) = r.component(self.id)?;

            // TODO: unwrap
            // TODO: Box (of raw value)
            let previous: Option<S> = if let Some(p) = prev {
                Some(serde_json::from_str(p.state.get()).unwrap())
            } else {
                None
            };
            let state = self.state.next_from_previous(previous);
            let state_serialized = serde_json::value::to_raw_value(&state).unwrap();

            let content_renderer = Renderer::new(); // TODO: use prev.hash_tree
            let view = (self.component)(state).await;
            let content_renderer = view.render(content_renderer).await?;
            r.write_u64(content_renderer.finish());
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
                r#"<server-component id="{}" data-id="{}"><script type="application/json">{}</script>{}</server-component>"#,
                id, self.id, initial, out.view
            )?;

            Ok(r)
        })
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: somehow do only once?
        f.write_str(if self.module.starts_with("r#") {
            &self.module[2..]
        } else {
            self.module
        })?;
        f.write_str("::")?;
        f.write_str(if self.name.starts_with("r#") {
            &self.name[2..]
        } else {
            self.name
        })?;
        Ok(())
    }
}
