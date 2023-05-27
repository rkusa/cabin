use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::value::RawValue;

use crate::component::Component;
use crate::render::PreviousComponent;
use crate::{Renderer, View, ViewHashTree};

pub struct Restored<C> {
    id: u32,
    component: C,
    previous_hash_tree: Option<ViewHashTree>,
}

impl<C> Restored<C>
where
    C: DeserializeOwned,
{
    pub(crate) fn restore(id: u32) -> Option<Self> {
        let previous = previous(id)?;
        // TODO: unwrap
        let component = serde_json::from_str(previous.state.get()).unwrap();
        Some(Self {
            id,
            component,
            previous_hash_tree: Some(previous.hash_tree),
        })
    }

    pub(crate) fn new(id: u32, component: C) -> Self {
        Self {
            id,
            component,
            previous_hash_tree: None,
        }
    }

    #[cfg(test)]
    pub fn with_previous_hash_tree(self, hash_tree: ViewHashTree) -> Self {
        Self {
            previous_hash_tree: Some(hash_tree),
            ..self
        }
    }

    pub fn map<T>(self, f: impl FnOnce(C) -> T) -> Restored<T> {
        Restored {
            id: self.id,
            component: f(self.component),
            previous_hash_tree: self.previous_hash_tree,
        }
    }
}

tokio::task_local! {
    pub(crate) static PREVIOUS: RefCell<Option<HashMap<u32, PreviousComponent>>>;
}

fn previous(id: u32) -> Option<PreviousComponent> {
    PREVIOUS
        .try_with(|previous| previous.borrow_mut().as_mut()?.remove(&id))
        .ok()?
}

impl<C> Deref for Restored<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<C> DerefMut for Restored<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.component
    }
}

impl<C, Ev> View<Ev> for Restored<C>
where
    C: Component + Serialize + Hash,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let mut component = r.component(C::id(), self.id);

        // TODO: unwrap
        let state_serialized = serde_json::value::to_raw_value(&self.component).unwrap();

        // Include state in hash to ensure state changes update the component (even if its view
        // doesn't change)
        self.component.hash(&mut component);

        write!(
            component,
            r#"<server-component id="{}" data-id="{}">"#,
            self.id,
            C::id(),
        )
        .map_err(crate::error::InternalError::from)?;

        let view = self.component.view().await.map_err(|err| err.into())?;
        let (mut r, hash_tree, changed) = component.content(view, self.previous_hash_tree).await?;

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
