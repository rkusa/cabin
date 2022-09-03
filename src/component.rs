use std::fmt::{self, Write};

use serde::Serialize;
use serde_json::value::RawValue;

use crate::view::ViewHash;
use crate::View;

// The conversion from View<A> to View<()> is the feature
// that ensures the usage of #[component]
pub struct Component<S, V: View<S>> {
    id: &'static str,
    state: S,
    render: fn(S) -> V,
}

impl<S, V: View<S>> Component<S, V> {
    pub fn new(id: &'static str, state: S, render: fn(S) -> V) -> Self {
        Component { id, state, render }
    }

    pub fn render_update(self) -> Result<(String, ViewHash), fmt::Error> {
        let mut result = String::new();
        let view = (self.render)(self.state);
        let view_hash = view.render(&mut result)?;
        let hash = view_hash.hash();
        Ok((result, view_hash.into_parent(hash)))
    }
}

impl<S: Serialize, V: View<S>> View<()> for Component<S, V> {
    fn render(self, mut out: impl Write) -> Result<ViewHash, fmt::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Initial {
            state: Box<RawValue>,
            view_hash: ViewHash,
        }

        // TODO: unwrap
        let state = serde_json::value::to_raw_value(&self.state).unwrap();
        let view = (self.render)(self.state);
        let mut inner = String::new();
        let view_hash = view.render(&mut inner)?;

        let hash = view_hash.hash();
        let initial = Initial {
            state,
            view_hash: view_hash.into_parent(hash),
        };
        let state = serde_json::to_string(&initial).unwrap();

        write!(out, r#"<server-component data-hash="{}">"#, hash)?;
        write!(out, r#"<script type="application/json">{}</script>"#, state)?;
        write!(out, r#"{}</server-component>"#, inner)?;

        Ok(initial.view_hash)
    }
}
