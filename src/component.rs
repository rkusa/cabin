pub mod registry;

use std::fmt::{self, Write};
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::value::RawValue;
use twox_hash::XxHash32;

use crate::previous::FromPrevious;
use crate::render::Renderer;
use crate::view::View;
use crate::ViewHashTree;

pub struct ServerComponent<F, V, P, S, E> {
    id: ComponentId,
    state: P,
    component: fn(S) -> F,
    marker: PhantomData<(V, P, E)>,
}

#[derive(Clone, Copy, Hash)]
pub struct ComponentId {
    module: &'static str,
    name: &'static str,
}

impl<F, V, P, S, E> ServerComponent<F, V, P, S, E> {
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

impl<F, V, P, S, E> View for ServerComponent<F, V, P, S, E>
where
    F: Future<Output = Result<V, E>>,
    V: View,
    crate::Error: From<E>,
    P: FromPrevious<S>,
    S: Default + Hash + Serialize + DeserializeOwned,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let instance_id = {
            let mut hasher = XxHash32::default();
            self.id.hash(&mut hasher);
            self.state.id().hash(&mut hasher);
            hasher.finish() as u32
        };
        let mut component = r.component(self.id, instance_id);

        // TODO: unwrap
        let previous_state = component.previous_state().unwrap();
        let state = self.state.next_from_previous(previous_state);
        let state_serialized = serde_json::value::to_raw_value(&state).unwrap();

        // Include state in hash to ensure state changes update the component (even if its view
        // doesn't change)
        state.hash(&mut component);

        write!(
            component,
            r#"<server-component id="{}" data-id="{}">"#,
            component.id(),
            self.id,
        )
        .map_err(crate::error::InternalError::from)?;

        let view = (self.component)(state).await?;
        let (mut r, hash_tree, changed) = component.content(view).await?;

        // If changed, add updated state and hash tree to output
        if changed {
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Initial<'a> {
                state: Box<RawValue>,
                hash_tree: &'a ViewHashTree,
            }
            let initial = serde_json::to_string(&Initial {
                state: state_serialized,
                hash_tree: &hash_tree,
            })
            .unwrap();

            write!(
                r,
                r#"<script type="application/json">{initial}</script></server-component>"#
            )
            .map_err(crate::error::InternalError::from)?;
        }

        Ok(r)
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
