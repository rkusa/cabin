pub mod registry;

use std::fmt::{self, Write};

use serde::Serialize;
use serde_json::value::RawValue;

use crate::view::hash::ViewHashTree;
use crate::view::HashTree;
use crate::{Render, View};

// The conversion from View<A> to View<()> is the feature
// that ensures the usage of #[component]
pub struct Component<S, V: View<S>, F: Fn(S) -> V> {
    module: &'static str,
    name: &'static str,
    state: S,
    render: F,
}

impl<S, V: View<S>, F: Fn(S) -> V> Component<S, V, F> {
    pub fn new(module: &'static str, name: &'static str, state: S, render: F) -> Self {
        Component {
            module,
            name,
            state,
            render,
        }
    }
}

impl<S1, S2: Serialize, V: View<S2>, F: Fn(S2) -> V> View<S1> for Component<S2, V, F> {
    type Renderer = ComponentRenderer<V::Renderer>;

    fn into_renderer(self, _hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        // TODO: unwrap
        // TODO: don't serialize if not rendered in the end
        let state = serde_json::value::to_raw_value(&self.state).unwrap();
        let mut hash_tree = HashTree::default();
        let content = (self.render)(self.state).into_renderer(&mut hash_tree);

        Some(ComponentRenderer {
            module: self.module,
            name: self.name,
            state,
            hash_tree: hash_tree.finish(),
            content,
        })
    }
}

pub struct ComponentRenderer<R> {
    module: &'static str,
    name: &'static str,
    state: Box<RawValue>,
    hash_tree: ViewHashTree,
    content: Option<R>,
}

impl<R> Render for ComponentRenderer<R>
where
    R: Render,
{
    fn render(&self, mut out: &mut dyn Write, is_update: bool) -> fmt::Result {
        if is_update {
            return self.content.as_ref().unwrap().render(&mut out, is_update); // TODO: unwrap fine?
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Initial<'a> {
            state: &'a RawValue,
            hash_tree: &'a ViewHashTree,
        }

        let initial = serde_json::to_string(&Initial {
            state: &self.state,
            hash_tree: &self.hash_tree,
        })
        .unwrap();

        write!(
            out,
            r#"<server-component data-id="{}::{}">"#,
            self.module, self.name
        )?;
        write!(
            out,
            r#"<script type="application/json">{}</script>"#,
            initial
        )?;
        self.content.as_ref().unwrap().render(&mut out, is_update)?; // TODO: unwrap fine?
        write!(out, r#"</server-component>"#)?;

        Ok(())
    }
}
