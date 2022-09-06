pub mod registry;

use std::fmt::{self, Write};

use serde::Serialize;
use serde_json::value::RawValue;

use crate::view::hash::ViewHashTree;
use crate::view::HashTree;
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

    pub fn render_update(self) -> Result<(String, ViewHashTree), fmt::Error> {
        let mut hash_tree = HashTree::default();
        let renderer = (self.render)(self.state).render(&mut hash_tree).unwrap(); // TODO: unwrap
        let mut result = String::new();
        renderer.render(&mut result)?;
        Ok((result, hash_tree.finish()))
    }
}

impl<S: Serialize, V: View<S>> View<()> for Component<S, V> {
    type Renderer = ComponentRenderer<V::Renderer>;

    fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Initial {
            state: Box<RawValue>,
            view_hash: ViewHashTree,
        }

        // TODO: unwrap
        // TODO: don't serialize if not rendered in the end
        let state = serde_json::value::to_raw_value(&self.state).unwrap();
        let content = (self.render)(self.state).render(hash_tree);

        let hash = hash_tree.hash();

        Some(ComponentRenderer {
            module: self.module,
            name: self.name,
            hash,
            state,
            content,
        })
    }
}

pub struct ComponentRenderer<R> {
    module: &'static str,
    name: &'static str,
    hash: u32,
    state: Box<RawValue>,
    content: Option<R>,
}

impl<R> Render for ComponentRenderer<R>
where
    R: Render,
{
    fn render(self, mut out: impl Write) -> fmt::Result {
        let state = serde_json::to_string(&self.state).unwrap();

        write!(
            out,
            r#"<server-component data-id="{}::{}" data-hash="{}">"#,
            self.module, self.name, self.hash
        )?;
        write!(out, r#"<script type="application/json">{}</script>"#, state)?;
        self.content.unwrap().render(&mut out)?; // TODO: unwrap fine?
        write!(out, r#"</server-component>"#)?;

        Ok(())
    }
}
