mod attributes;
pub mod elements;
pub mod events;

use std::borrow::Cow;

pub use attributes::Attributes;
pub use elements::*;
use serde::Serialize;

use self::attributes::Attribute;
use crate::actions::ActionsRegistry;
use crate::render::{is_void_element, Renderer};
use crate::signal::SignalMut;
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
    on_click: Option<&'static str>,
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
    pub fn on_click<T>(mut self, action: fn(SignalMut<T>)) -> Self
    where
        T: Serialize,
    {
        let name = ActionsRegistry::global().action_name(action as usize);
        debug_assert!(name.is_some(), "action not registered");

        if let Some(name) = name {
            // TODO: unwrap
            // TODO: delay serialization?
            self.on_click = Some(name);
        }

        self
    }
}

impl<V, A, K> View for Html<V, A, K>
where
    V: View,
    A: Attributes,
    K: Attributes,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let mut el = r.element(self.tag)?;

        if let Some(action_name) = self.on_click {
            el.attribute("cabin-click", action_name)
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
