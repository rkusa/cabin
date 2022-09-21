pub mod registry;

use std::fmt::{self, Write};

use serde::Serialize;
use serde_json::value::RawValue;

use crate::render::Renderer;
use crate::view::{IntoView, View};
use crate::ViewHashTree;

// The conversion from View<A> to View<()> is the feature
// that ensures the usage of #[component]
pub struct Component<S, V> {
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
        let mut r = Renderer::from_previous_tree(previous_tree);
        (self.render)(self.state).render(&mut r)?;
        let out = r.end();
        Ok((out.view, out.hash_tree))
    }
}

impl<S: Serialize + Clone, V: View<S>> View<()> for Component<S, V> {
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        let content = (self.render)(self.state.clone());

        if r.is_update() {
            return content.render(r);
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Initial<'a, S> {
            state: &'a S,
            hash_tree: &'a ViewHashTree,
        }

        let mut content_renderer = Renderer::new();
        content.render(&mut content_renderer)?;
        let out = content_renderer.end();

        // TODO: unwrap
        let initial = serde_json::to_string(&Initial {
            state: &self.state,
            hash_tree: &out.hash_tree,
        })
        .unwrap();

        write!(
            r,
            r#"<server-component data-id="{}::{}"><script type="application/json">{}</script>{}</server-component>"#,
            self.module, self.name, initial, out.view
        )?;

        Ok(())
    }
}

impl<S: Serialize + Clone, V: View<S>> IntoView<Component<S, V>, ()> for Component<S, V> {
    fn into_view(self) -> Component<S, V> {
        self
    }
}
