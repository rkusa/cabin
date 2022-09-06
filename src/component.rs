pub mod registry;

use std::fmt::{self, Write};

use serde::Serialize;
use serde_json::value::RawValue;

use crate::view::hash::ViewHashTree;
use crate::view::HashTree;
use crate::View;

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
        let mut result = String::new();
        let mut hash_tree = HashTree::default();
        let view = (self.render)(self.state);
        let view_hash = view.render(&mut hash_tree, &mut result)?;
        Ok((result, hash_tree.finish()))
    }
}

impl<S: Serialize, V: View<S>> View<()> for Component<S, V> {
    fn render(self, _hash_tree: &mut HashTree, mut out: impl Write) -> fmt::Result {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Initial {
            state: Box<RawValue>,
            view_hash: ViewHashTree,
        }

        // TODO: unwrap
        let state = serde_json::value::to_raw_value(&self.state).unwrap();
        let view = (self.render)(self.state);
        let mut hash_tree = HashTree::default();
        let mut inner = String::new();
        view.render(&mut hash_tree, &mut inner)?;

        let hash = hash_tree.hash();
        let state = serde_json::to_string(&state).unwrap();

        write!(
            out,
            r#"<server-component data-id="{}::{}" data-hash="{}">"#,
            self.module, self.name, hash
        )?;
        write!(out, r#"<script type="application/json">{}</script>"#, state)?;
        write!(out, r#"{}</server-component>"#, inner)?;

        eprintln!("{:?}", hash_tree.finish());

        Ok(())
    }
}
