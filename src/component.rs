pub mod registry;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::future::Future;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::render::Renderer;
use crate::view::{IntoView, View};
use crate::ViewHashTree;

pub trait ServerComponent: Component {
    fn id() -> Cow<'static, str>;
}

// TODO: s/DeserializeOwned/Deserialize/ using GATs?
pub trait Component: Serialize + DeserializeOwned {
    type Message<'v>: Serialize + Deserialize<'v>;
    type View<'v>: View<Self::Message<'v>>
    where
        Self: 'v;

    type UpdateFuture<'v>: Future<Output = ()> + Send + 'v
    where
        Self: 'v;
    type RenderFuture<'v>: Future<Output = Self::View<'v>> + Send + 'v
    where
        Self: 'v;

    fn update(&mut self, message: Self::Message<'_>) -> Self::UpdateFuture<'_>;
    fn render(&self) -> Self::RenderFuture<'_>;
}

pub struct ComponentView<C>
where
    C: ServerComponent,
{
    component: C,
}

impl<C> IntoView<ComponentView<C>, ()> for C
where
    C: ServerComponent + Send + 'static,
    for<'v> <C as Component>::View<'v>: Send,
{
    fn into_view(self) -> ComponentView<C> {
        ComponentView { component: self }
    }
}

impl<C> View<()> for ComponentView<C>
where
    C: ServerComponent + Send + 'static,
    for<'v> <C as Component>::View<'v>: Send,
{
    type Future = impl Future<Output = Result<Renderer, fmt::Error>> + Send;

    fn render(self, mut r: Renderer) -> Self::Future {
        async move {
            if r.is_update() {
                let c = self.component.render();
                return c.await.render(r).await;
            }

            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct Initial<'a, S> {
                state: &'a S,
                hash_tree: &'a ViewHashTree,
            }

            let mut content_renderer = Renderer::new();
            let c = self.component.render();
            content_renderer = c.await.render(content_renderer).await?;
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

            Ok(r)
        }
    }
}
