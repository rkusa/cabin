mod attributes;
pub mod elements;
pub mod events;

use std::any::TypeId;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};

pub use attributes::Attributes;
#[doc(inline)]
pub use elements::anchor::a;
#[doc(inline)]
pub use elements::old::*;
use serde::Serialize;
use twox_hash::XxHash32;

use self::attributes::Attribute;
use crate::render::{is_void_element, Renderer};
pub use crate::view::text::{text, Text};
use crate::view::View;

pub fn custom<V: View>(tag: &'static str, content: V) -> Html<V, (), ()> {
    Html {
        tag,
        attrs: (),
        on_click: None,
        kind: (),
        content,
    }
}

pub fn create<V: View, K: Default>(tag: &'static str, content: V) -> Html<V, (), K> {
    Html {
        tag,
        attrs: (),
        on_click: None,
        kind: K::default(),
        content,
    }
}

pub struct Html<V, A, K> {
    tag: &'static str,
    attrs: A,
    // TODO: no box?
    on_click: Option<Box<dyn FnOnce() -> (u32, String)>>,
    kind: K,
    content: V,
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

impl<V, A, K> View for Html<V, A, K>
where
    V: View,
    A: Attributes,
    K: Attributes,
{
    async fn render(self, r: Renderer, include_hash: bool) -> Result<Renderer, crate::Error> {
        let mut el = r.element(self.tag, include_hash)?;

        if let Some(event) = self.on_click {
            // TODO: directly write into el?
            let (id, payload) = &(event)();
            el.attribute("cabin-click", id)
                .map_err(crate::error::InternalError::from)?;
            el.attribute("cabin-click-payload", payload)
                .map_err(crate::error::InternalError::from)?;
        }

        self.attrs.render(&mut el)?;
        self.kind.render(&mut el)?;

        if !is_void_element(self.tag) {
            el.content(self.content).await
        } else {
            el.end()
        }
    }

    fn prime(&mut self) {
        self.content.prime();
    }
}
