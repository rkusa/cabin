mod attributes;
pub mod events;
mod macros;

use std::borrow::Cow;
use std::fmt;
use std::future::Future;
use std::pin::Pin;

use serde::de::DeserializeOwned;
use serde::Serialize;

use self::attributes::{Attribute, Attributes};
use self::events::InputEvent;
use crate::component::registry::ComponentRegistry;
use crate::render::{is_void_element, Renderer};
pub use crate::view::text::{text, Text};
use crate::view::{IntoView, View, ViewWrapper};
pub use macros::*;

pub fn div<V: View>(content: impl IntoView<V>) -> Html<V, ()> {
    custom("div", content)
}

pub fn ul<V: View>(content: impl IntoView<V>) -> Html<V, ()> {
    custom("ul", content)
}

pub fn li<V: View>(content: impl IntoView<V>) -> Html<V, ()> {
    custom("li", content)
}

pub fn fieldset<V: View>(content: impl IntoView<V>) -> Html<V, ()> {
    custom("fieldset", content)
}

pub fn button<V: View>(content: impl IntoView<V>) -> Html<V, ()> {
    custom("button", content)
}

pub fn input() -> Html<ViewWrapper<()>, ()> {
    custom("input", ())
}

pub fn custom<V: View>(tag: &'static str, content: impl IntoView<V>) -> Html<V, ()> {
    Html {
        tag: HtmlTag {
            tag,
            attrs: (),
            on_click: None,
            on_input: None,
        },
        content: content.into_view(),
    }
}

struct HtmlTag<A> {
    tag: &'static str,
    attrs: A,
    on_click: Option<(&'static str, String)>,
    on_input: Option<&'static str>,
}

pub struct Html<V, A> {
    tag: HtmlTag<A>,
    content: V,
}

impl<V, A> Html<V, A> {
    pub fn attr<'a>(
        self,
        name: &'static str,
        value: impl Into<Cow<'a, str>>,
    ) -> Html<V, impl Attributes + 'a>
    where
        A: Attributes + 'a,
    {
        Html {
            tag: HtmlTag {
                tag: self.tag.tag,
                attrs: Attribute::new(name, value, self.tag.attrs),
                on_click: self.tag.on_click,
                on_input: self.tag.on_input,
            },
            content: self.content,
        }
    }

    // TODO: not available for all tags (e.g. only for buttons)
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
            self.tag.on_click = Some((name, payload));
        }
        self
    }

    // TODO: not available for all tags (e.g. only for inputs)
    pub fn on_input<M, F: Future<Output = M>>(mut self, action: fn(M, InputEvent) -> F) -> Self {
        let name = ComponentRegistry::global().action_name(action as usize);
        debug_assert!(name.is_some(), "action not registered");
        self.tag.on_input = name;
        self
    }
}

impl<V, A> View for Html<V, A>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    V: View + Send + 'static,
    A: Attributes + Send + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async move {
            let mut el = r.element(self.tag.tag)?;

            if let Some((on_click, payload)) = &self.tag.on_click {
                el.attribute("data-click", on_click)?;
                el.attribute("data-click-payload", payload)?;
            }

            if let Some(on_input) = &self.tag.on_input {
                el.attribute("data-input", on_input)?;
            }

            self.tag.attrs.render(&mut el)?;

            if !is_void_element(self.tag.tag) {
                el.content(self.content).await
            } else {
                el.end()
            }
        })
    }
}
