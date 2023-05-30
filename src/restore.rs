use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::value::RawValue;
use tokio::task::JoinHandle;
use twox_hash::XxHash32;

use crate::component::Component;
use crate::render::PreviousComponent;
use crate::view::BoxedView;
use crate::{Renderer, View, ViewHashTree};

pub struct Restored<C> {
    id: u32,
    component: ComponentState<C>,
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
            component: ComponentState::Stored(component),
            previous_hash_tree: Some(previous.hash_tree),
        })
    }

    pub(crate) fn new(id: u32, component: C) -> Self {
        Self {
            id,
            component: ComponentState::Stored(component),
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
            component: match self.component {
                ComponentState::Stored(component) => ComponentState::Stored(f(component)),
                ComponentState::Primed { .. } | ComponentState::Intermediate => unreachable!(),
            },
            previous_hash_tree: self.previous_hash_tree,
        }
    }
}

enum ComponentState<C> {
    Stored(C),
    Primed {
        hash: u32,
        state_serialized: Box<RawValue>,
        view: JoinHandle<Result<BoxedView<()>, crate::Error>>,
    },
    Intermediate,
}

impl<C> ComponentState<C>
where
    C: Component + Hash + Serialize + 'static,
{
    fn prime(self) -> Self {
        match self {
            ComponentState::Primed { .. } => self,
            ComponentState::Stored(component) => {
                // TODO: unwrap
                let state_serialized = serde_json::value::to_raw_value(&component).unwrap();

                // Include state in hash to ensure state changes update the component (even if its view
                // doesn't change)
                let mut hasher = XxHash32::default();
                component.hash(&mut hasher);
                let hash = hasher.finish() as u32;

                let view = tokio::task::spawn_local(async {
                    component
                        .view()
                        .await
                        .map(|mut view| {
                            view.prime();
                            view.coerce().boxed()
                        })
                        .map_err(|err| err.into())
                });
                ComponentState::Primed {
                    hash,
                    state_serialized,
                    view,
                }
            }
            ComponentState::Intermediate => unreachable!(),
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
        match &self.component {
            ComponentState::Stored(component) => component,
            ComponentState::Primed { .. } | ComponentState::Intermediate => unreachable!(),
        }
    }
}

impl<C> DerefMut for Restored<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match &mut self.component {
            ComponentState::Stored(component) => component,
            ComponentState::Primed { .. } | ComponentState::Intermediate => unreachable!(),
        }
    }
}

impl<C> View for Restored<C>
where
    C: Component + Serialize + Hash + 'static,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let component = self.component.prime();
        let ComponentState::Primed { hash, state_serialized, view } = component else {
            unreachable!();
        };

        let mut component = r.component(C::id(), self.id);

        // Include state in hash to ensure state changes update the component (even if its view
        // doesn't change)
        component.write_u32(hash);

        write!(
            component,
            r#"<server-component id="{}" data-id="{}">"#,
            self.id,
            C::id(),
        )
        .map_err(crate::error::InternalError::from)?;

        // TODO: handle JoinError?
        let view = view.await.unwrap()?;
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

    fn prime(&mut self) {
        let s = std::mem::replace(&mut self.component, ComponentState::Intermediate);
        let _ = std::mem::replace(&mut self.component, s.prime());
    }
}
