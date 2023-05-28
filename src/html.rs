mod attributes;
pub mod elements;
pub mod events;

use std::borrow::Cow;

pub use attributes::Attributes;
pub use elements::*;
use serde::Serialize;

use self::attributes::Attribute;
use crate::render::{is_void_element, Renderer};
pub use crate::view::text::{text, Text};
use crate::view::View;

pub fn custom<V: View<Ev>, Ev>(tag: &'static str, content: V) -> Html<V, Ev, (), ()> {
    Html {
        tag,
        attrs: (),
        on_click: None,
        kind: (),
        content,
    }
}

pub fn create<V: View<Ev>, Ev, K: Default>(tag: &'static str, content: V) -> Html<V, Ev, (), K> {
    Html {
        tag,
        attrs: (),
        on_click: None,
        kind: K::default(),
        content,
    }
}

pub struct Html<V, Ev, A, K> {
    tag: &'static str,
    attrs: A,
    on_click: Option<Ev>,
    kind: K,
    content: V,
}

impl<V, Ev, A, K> Html<V, Ev, A, K> {
    pub fn attr<'a>(
        self,
        name: &'static str,
        value: impl Into<Cow<'a, str>>,
    ) -> Html<V, Ev, impl Attributes + 'a, K>
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

    pub fn class<'a>(self, value: impl Into<Cow<'a, str>>) -> Html<V, Ev, impl Attributes + 'a, K>
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

    pub fn on_click(mut self, event: Ev) -> Self {
        self.on_click = Some(event);
        self
    }
}

impl<V, Ev, A, K> View<Ev> for Html<V, Ev, A, K>
where
    V: View<Ev>,
    Ev: Serialize,
    A: Attributes,
    K: Attributes,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        let mut el = r.element(self.tag)?;

        if let Some(event) = self.on_click {
            // TODO: unwrap
            let event = serde_json::to_string(&event).unwrap();
            el.attribute("data-click", &event)
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
}
