mod attributes;
pub mod elements;
pub mod events;
mod fragment;
mod macros;

use std::borrow::Cow;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

pub use elements::*;
pub use macros::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use self::attributes::{Attribute, Attributes};
use crate::component::registry::ComponentRegistry;
use crate::render::{is_void_element, Renderer};
pub use crate::view::text::{text, Text};
use crate::view::{IntoView, View};
pub use fragment::Fragment;

pub fn custom<V: View>(tag: &'static str, content: impl IntoView<V>) -> Html<V, (), ()> {
    Html {
        tag,
        attrs: (),
        on_click: None,
        kind: (),
        content: content.into_view(),
    }
}

fn create<V: View, K>(tag: &'static str, kind: K, content: V) -> Html<V, (), K> {
    Html {
        tag,
        attrs: (),
        on_click: None,
        kind,
        content,
    }
}

pub struct Html<V, A, K> {
    tag: &'static str,
    attrs: A,
    on_click: Option<(&'static str, String)>,
    kind: K,
    content: V,
}

pub trait AddChild<C = Self>
where
    C: View,
{
    type Output;

    fn add_child(self, child: C) -> Self::Output;
}

impl<V, A, K> Html<V, A, K> {
    pub fn attr<'a>(
        self,
        name: &'static str,
        value: impl Into<Cow<'a, str>>,
    ) -> Html<V, impl Attributes + 'a, K>
    where
        A: Attributes + 'a,
    {
        Html {
            tag: self.tag,
            attrs: Attribute::new(name, value, self.attrs),
            on_click: self.on_click,
            kind: self.kind,
            content: self.content,
        }
    }

    pub fn class<'a>(self, value: impl Into<Cow<'a, str>>) -> Html<V, impl Attributes + 'a, K>
    where
        A: Attributes + 'a,
    {
        Html {
            tag: self.tag,
            attrs: Attribute::new("class", value, self.attrs),
            on_click: self.on_click,
            kind: self.kind,
            content: self.content,
        }
    }

    pub fn on_click<M, F: Future<Output = M>, P: Serialize + DeserializeOwned>(
        mut self,
        action: fn(M, P) -> F,
        payload: P,
    ) -> Self {
        let name = ComponentRegistry::global().action_name(action as usize);
        debug_assert!(name.is_some(), "action not registered");

        if let Some(name) = name {
            // TODO: unwrap
            // TODO: delay serialization?
            let payload = serde_json::to_string(&payload).unwrap();
            self.on_click = Some((name, payload));
        }
        self
    }
}

impl<V, A, K> View for Html<V, A, K>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    V: View + Send + 'static,
    A: Attributes + Send + 'static,
    K: Attributes + Send + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async move {
            let mut el = r.element(self.tag)?;

            if let Some((on_click, payload)) = &self.on_click {
                el.attribute("data-click", on_click)?;
                el.attribute("data-click-payload", payload)?;
            }

            self.attrs.render(&mut el)?;
            self.kind.render(&mut el)?;

            if !is_void_element(self.tag) {
                el.content(self.content).await
            } else {
                el.end()
            }
        })
    }
}
