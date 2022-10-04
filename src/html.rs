mod attributes;
pub mod events;

use std::borrow::Cow;
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;

use self::attributes::{Attribute, Attributes};
use self::events::InputEvent;
use crate::component::registry::ComponentRegistry;
use crate::render::{is_void_element, Renderer};
use crate::view::{IntoView, View};

pub fn div<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("div", content)
}

pub fn ul<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("ul", content)
}

pub fn li<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("li", content)
}

pub fn button<V: View<M>, M>(content: impl IntoView<V, M>) -> Html<V, M, ()> {
    custom("button", content)
}

pub fn input<M>() -> Html<(), M, ()> {
    custom("input", ())
}

pub fn custom<V: View<M>, M>(tag: &'static str, content: impl IntoView<V, M>) -> Html<V, M, ()> {
    Html {
        tag: HtmlTag {
            tag,
            attrs: (),
            on_click: None,
            on_input: None,
            marker: PhantomData,
        },
        content: content.into_view(),
    }
}

struct HtmlTag<M, A> {
    tag: &'static str,
    attrs: A,
    on_click: Option<(&'static str, String)>,
    on_input: Option<&'static str>,
    marker: PhantomData<M>,
}

pub struct Html<V, M, A> {
    tag: HtmlTag<M, A>,
    content: V,
}

impl<V, M, A> Html<V, M, A> {
    pub fn attr<'a>(
        self,
        name: &'static str,
        value: impl Into<Cow<'a, str>>,
    ) -> Html<V, M, impl Attributes + 'a>
    where
        A: Attributes + 'a,
    {
        Html {
            tag: HtmlTag {
                tag: self.tag.tag,
                attrs: Attribute::new(name, value, self.tag.attrs),
                on_click: self.tag.on_click,
                on_input: self.tag.on_input,
                marker: PhantomData,
            },
            content: self.content,
        }
    }

    // TODO: not available for all tags (e.g. only for buttons)
    pub fn on_click<F: Future<Output = M>, P: Serialize + DeserializeOwned>(
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
    pub fn on_input<F: Future<Output = M>>(mut self, action: fn(M, InputEvent) -> F) -> Self {
        let name = ComponentRegistry::global().action_name(action as usize);
        debug_assert!(name.is_some(), "action not registered");
        self.tag.on_input = name;
        self
    }
}

impl<V, M, A> View<M> for Html<V, M, A>
where
    V: View<M> + Send,
    M: Serialize + Send,
    A: Attributes + Send,
{
    type Future = impl Future<Output = Result<Renderer, fmt::Error>> + Send;

    fn render(self, r: Renderer) -> Self::Future {
        async move {
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
        }
    }
}

impl<V, M, A> IntoView<Html<V, M, A>, M> for Html<V, M, A>
where
    V: View<M> + Send,
    M: Serialize + Send,
    A: Attributes + Send,
{
    fn into_view(self) -> Html<V, M, A> {
        self
    }
}
