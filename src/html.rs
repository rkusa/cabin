mod element_ext;
pub mod elements;
pub mod events;

use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub use element_ext::ElementExt;
#[doc(inline)]
pub use elements::anchor::a;
#[doc(inline)]
pub use elements::form::form;
#[doc(inline)]
pub use elements::label::label;
#[doc(inline)]
pub use elements::nav::nav;
#[doc(inline)]
pub use elements::old::*;
#[doc(inline)]
pub use elements::span::span;
#[doc(inline)]
pub use elements::time::time;
use serde::Serialize;
use twox_hash::XxHash32;

use self::elements::global::Global;
use crate::render::{is_void_element, Renderer};
pub use crate::view::text::{text, Text};
use crate::view::{RenderFuture, View};

pub struct Html<V, K> {
    tag: &'static str,
    class: Option<Cow<'static, str>>,
    attrs: Option<HashMap<&'static str, Cow<'static, str>>>,
    // TODO: no box?
    on_click: Option<Box<dyn FnOnce() -> (u32, String)>>,
    global: Global,
    kind: K,
    content: V,
}

pub fn custom<V: View>(tag: &'static str, content: V) -> Html<V, ()> {
    Html::new(tag, content)
}

impl<V, K> Html<V, K> {
    pub fn new(tag: &'static str, content: V) -> Html<V, K>
    where
        V: View,
        K: Default,
    {
        Html {
            tag,
            attrs: None,
            class: None,
            on_click: None,
            global: Default::default(),
            kind: K::default(),
            content,
        }
    }

    pub fn attr(mut self, name: &'static str, value: impl Into<Cow<'static, str>>) -> Html<V, K> {
        // TODO: replace with `get_or_insert_default();` once stable
        let attrs = match self.attrs.as_mut() {
            Some(attrs) => attrs,
            None => {
                self.attrs = Some(Default::default());
                self.attrs.as_mut().unwrap()
            }
        };
        attrs.insert(name, value.into());
        self
    }

    pub fn class(mut self, class: impl Into<Cow<'static, str>>) -> Html<V, K> {
        self.class = Some(class.into());
        self
    }

    // TODO: multiple arguments for action
    pub fn on_click<E>(mut self, event: E) -> Self
    where
        E: Serialize + 'static,
    {
        self.on_click = Some(Box::new(move || {
            let mut hasher = XxHash32::default();
            TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;

            // TODO: unwrap
            (hash, serde_json::to_string(&event).unwrap())
        }));

        self
    }
}

impl<V, K> View for Html<V, K>
where
    V: View + 'static,
    K: ElementExt + 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        RenderFuture::Future(Box::pin(async move {
            let Html {
                tag,
                attrs,
                on_click,
                class,
                global,
                kind,
                content,
            } = self;

            let mut el = r.element(tag, include_hash)?;

            if let Some(event) = on_click {
                // TODO: directly write into el?
                let (id, payload) = &(event)();
                el.attribute("cabin-click", id)
                    .map_err(crate::error::InternalError::from)?;
                el.attribute("cabin-click-payload", payload)
                    .map_err(crate::error::InternalError::from)?;
            }

            if let Some(class) = class {
                el.attribute("class", class)
                    .map_err(crate::error::InternalError::from)?;
            }

            if let Some(attrs) = attrs {
                for (name, value) in attrs {
                    el.attribute(name, value)
                        .map_err(crate::error::InternalError::from)?;
                }
            }

            global.render(&mut el)?;
            kind.render(&mut el)?;

            if !is_void_element(tag) {
                el.content(content).await
            } else {
                el.end()
            }
        }))
    }

    fn prime(&mut self) {
        self.content.prime();
    }
}
