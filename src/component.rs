pub mod id;
pub mod registry;

use std::fmt::{self, Write};
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;

use serde::Serialize;
use serde_json::value::RawValue;

use crate::component::id::NanoId;
use crate::render::Renderer;
use crate::view::View;
use crate::ViewHashTree;

pub struct ServerComponent<F, V, S> {
    id: ComponentId,
    state: S,
    component: fn(S) -> F,
    marker: PhantomData<V>,
}

#[derive(Clone, Copy, Hash)]
pub struct ComponentId {
    module: &'static str,
    name: &'static str,
}

impl<F, V, S> ServerComponent<F, V, S> {
    pub fn new(id: ComponentId, state: S, component: fn(S) -> F) -> Self {
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

impl<F, V, S> View for ServerComponent<F, V, S>
where
    F: Future<Output = V> + Send + 'static,
    V: View + Send + 'static,
    S: Serialize + Send + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, mut r: Renderer) -> Self::Future {
        Box::pin(async move {
            let id = NanoId::random();
            if !r.component(self.id, id)? {
                // TODO: nested update
                // TODO: nested no-update in list
                // Components are self-contained by default and parent re-renders are ignored.
                return Ok(r);
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
