pub mod registry;

use std::borrow::Cow;
use std::fmt::{self, Write};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::render::Renderer;
use crate::view::{IntoView, View};
use crate::ViewHashTree;

pub trait Component: Render {
    fn id() -> Cow<'static, str>;
}

// TODO: s/DeserializeOwned/Deserialize/ using GATs?
pub trait Render: Serialize + DeserializeOwned {
    type Message<'v>: Serialize + Deserialize<'v>;
    type View<'v>: View<Self::Message<'v>>
    where
        Self: 'v;

    fn update(&mut self, _message: Self::Message<'_>) {}
    fn render(&self) -> Self::View<'_>;
}

pub struct ComponentView<C>
where
    C: Component,
{
    component: C,
}

impl<C> IntoView<ComponentView<C>, ()> for C
where
    C: Component,
{
    fn into_view(self) -> ComponentView<C> {
        ComponentView { component: self }
    }
}

impl<C> View<()> for ComponentView<C>
where
    C: Component,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        if r.is_update() {
            return self.component.render().render(r);
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Initial<'a, S> {
            state: &'a S,
            hash_tree: &'a ViewHashTree,
        }

        let mut content_renderer = Renderer::new();
        self.component.render().render(&mut content_renderer)?;
        let out = content_renderer.end();

        // TODO: unwrap
        let initial = serde_json::to_string(&Initial::<C> {
            state: &self.component,
            hash_tree: &out.hash_tree,
        })
        .unwrap();

        write!(
            r,
            r#"<server-component data-id="{}"><script type="application/json">{}</script>{}</server-component>"#,
            C::id(),
            initial,
            out.view
        )?;

        Ok(())
    }
}
