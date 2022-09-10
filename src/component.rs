pub mod registry;

use std::fmt::{self, Write};

use serde::Serialize;
use serde_json::value::RawValue;

use crate::view::hash::ViewHashTree;
use crate::view::{HashTree, IntoView};
use crate::{Render, View};

// The conversion from View<A> to View<()> is the feature
// that ensures the usage of #[component]
pub struct Component<S, V: View<S>> {
    module: &'static str,
    name: &'static str,
    state: S,
    render: fn(S) -> V,
}

impl<S, V: View<S>> Component<S, V> {
    pub fn new(module: &'static str, name: &'static str, state: S, render: fn(S) -> V) -> Self {
        Component {
            module,
            name,
            state,
            render,
        }
    }

    pub fn render_update(
        self,
        previous_tree: ViewHashTree,
    ) -> Result<(String, ViewHashTree), fmt::Error> {
        let mut hash_tree = HashTree::from_previous_tree(previous_tree);
        let renderer = (self.render)(self.state).prepare(&mut hash_tree).unwrap(); // TODO: unwrap
        let mut result = String::new();
        // TODO: remove `is_update` again?
        renderer.render(&mut result, true)?;
        Ok((result, hash_tree.finish()))
    }
}

impl<S: Serialize, V: View<S>> View<()> for Component<S, V> {
    type Render = ComponentRenderer<V::Render>;

    fn prepare(self, _hash_tree: &mut HashTree) -> Option<Self::Render> {
        // TODO: unwrap
        // TODO: don't serialize if not rendered in the end
        let state = serde_json::value::to_raw_value(&self.state).unwrap();
        let mut hash_tree = HashTree::default();
        let content = (self.render)(self.state).prepare(&mut hash_tree);

        Some(ComponentRenderer {
            module: self.module,
            name: self.name,
            state,
            hash_tree: hash_tree.finish(),
            content,
        })
    }
}

impl<S: Serialize, V: View<S>> IntoView<Component<S, V>, ()> for Component<S, V> {
    fn into_view(self) -> Component<S, V> {
        self
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
