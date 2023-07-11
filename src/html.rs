mod element_ext;
pub mod elements;
pub mod events;
pub mod list;
mod raw;

use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub use element_ext::ElementExt;
#[doc(inline)]
pub use exports::*;
pub use raw::{raw, Raw};
use serde::Serialize;
use twox_hash::XxHash32;

use self::elements::aria::Aria;
use self::elements::global::Global;
use crate::error::InternalError;
use crate::render::Renderer;
use crate::view::{RenderFuture, View};

mod exports {
    #[doc(inline)]
    pub use super::elements::anchor::a;
    #[doc(inline)]
    pub use super::elements::body::body;
    #[doc(inline)]
    pub use super::elements::button::button;
    #[doc(inline)]
    pub use super::elements::dialog::dialog;
    #[doc(inline)]
    pub use super::elements::div::div;
    #[doc(inline)]
    pub use super::elements::form::form;
    #[doc(inline)]
    pub use super::elements::head::head;
    #[doc(inline)]
    pub use super::elements::html::html;
    #[doc(inline)]
    pub use super::elements::label::label;
    #[doc(inline)]
    pub use super::elements::link::link;
    #[doc(inline)]
    pub use super::elements::nav::nav;
    #[doc(inline)]
    pub use super::elements::script::script;
    #[doc(inline)]
    pub use super::elements::span::span;
    #[doc(inline)]
    pub use super::elements::time::time;
    #[doc(inline)]
    pub use crate::view::text::{text, Text};

    pub fn doctype() -> impl crate::View {
        super::raw("<!DOCTYPE html>")
    }
}

pub struct Html<V, K> {
    tag: &'static str,
    id: Option<Cow<'static, str>>,
    class: Option<Cow<'static, str>>,
    attrs: Option<HashMap<&'static str, Cow<'static, str>>>,
    on_click: Option<Box<SerializeEventFn>>,
    // Boxed to not blow up struct size.
    global: Option<Box<Global>>,
    aria: Option<Box<Aria>>,
    kind: K,
    content: V,
}

pub(crate) type SerializeEventFn = dyn FnOnce() -> Result<(u32, String), InternalError>;

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
            id: None,
            attrs: None,
            class: None,
            on_click: None,
            global: Default::default(),
            aria: Default::default(),
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

    /// Unique identifier across the document.
    pub fn id(mut self, id: impl Into<Cow<'static, str>>) -> Html<V, K> {
        self.id = Some(id.into());
        self
    }

    /// The various classes that the element belongs to.
    pub fn class(mut self, class: impl Into<Cow<'static, str>>) -> Html<V, K> {
        self.class = Some(class.into());
        self
    }

    /// Append classes that the element belongs to.
    pub fn add_class(mut self, class: impl Into<Cow<'static, str>>) -> Html<V, K> {
        self.class = match self.class {
            Some(before) => Some(format!("{} {}", before, class.into()).into()),
            None => Some(class.into()),
        };
        self
    }

    pub fn on_click<E>(mut self, event: E) -> Self
    where
        E: Serialize + 'static,
    {
        self.on_click = Some(Box::new(move || {
            let mut hasher = XxHash32::default();
            TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_click event",
                    err,
                })
                .map(|json| (hash, json))
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
                id,
                class,
                global,
                aria,
                kind,
                content,
            } = self;

            let mut el = r.element(tag, include_hash)?;

            if let Some(event) = on_click {
                // TODO: directly write into el?
                let (id, payload) = &(event)()?;
                el.attribute("cabin-click", id)
                    .map_err(crate::error::InternalError::from)?;
                el.attribute("cabin-click-payload", payload)
                    .map_err(crate::error::InternalError::from)?;
            }

            if let Some(id) = id {
                el.attribute("id", id)
                    .map_err(crate::error::InternalError::from)?;
            }

            if let Some(class) = class {
                el.attribute("class", class)
                    .map_err(crate::error::InternalError::from)?;
            }

            if let Some(attrs) = attrs {
                for (name, value) in attrs {
                    if !valid_attribute_name(name) {
                        return Err(InternalError::InvalidAttributeName {
                            name: name.to_string(),
                        }
                        .into());
                    }
                    el.attribute(name, value)
                        .map_err(crate::error::InternalError::from)?;
                }
            }

            if let Some(global) = global {
                global.render(&mut el)?;
            }
            if let Some(aria) = aria {
                aria.render(&mut el)?;
            }
            kind.render(&mut el)?;

            if !K::is_void_element() {
                el.content(content).await
            } else {
                el.end(true)
            }
        }))
    }

    fn prime(&mut self) {
        self.content.prime();
    }
}

fn valid_attribute_name(name: &str) -> bool {
    // https://html.spec.whatwg.org/multipage/syntax.html#attributes-2
    !name.chars().any(|ch| {
        matches!(ch,
            ' ' | '"' | '\'' | '>' | '/' | '=' |
            /* controls */
            '\u{7F}'..='\u{9F}' |
            /* non character */
            '\u{FDD0}'..='\u{FDEF}' |  '\u{FFFE}' | '\u{FFFF}' | '\u{1FFFE}' | '\u{1FFFF}' |
            '\u{2FFFE}' | '\u{2FFFF}' | '\u{3FFFE}' | '\u{3FFFF}' | '\u{4FFFE}' | '\u{4FFFF}' |
            '\u{5FFFE}' | '\u{5FFFF}' | '\u{6FFFE}' | '\u{6FFFF}' | '\u{7FFFE}' | '\u{7FFFF}' |
            '\u{8FFFE}' | '\u{8FFFF}' | '\u{9FFFE}' | '\u{9FFFF}' | '\u{AFFFE}' | '\u{AFFFF}' |
            '\u{BFFFE}' | '\u{BFFFF}' | '\u{CFFFE}' | '\u{CFFFF}' | '\u{DFFFE}' | '\u{DFFFF}' |
            '\u{EFFFE}' | '\u{EFFFF}' | '\u{FFFFE}' | '\u{FFFFF}' | '\u{10FFFE}' |  '\u{10FFFF}'
        )
    })
}
