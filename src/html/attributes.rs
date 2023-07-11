use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use serde::Serialize;
use twox_hash::XxHash32;

use crate::error::InternalError;
use crate::html::elements::aria::Aria;
use crate::html::elements::global::Global;
use crate::render::ElementRenderer;

use super::elements::ElementExt;

#[derive(Default)]
pub struct Attributes<El, Ext> {
    pub id: Option<Cow<'static, str>>,
    pub class: Option<Cow<'static, str>>,
    pub on_click: Option<Box<SerializeEventFn>>,
    // Boxed to not blow up struct size.
    pub global: Option<Box<Global>>,
    pub aria: Option<Box<Aria>>,
    pub custom: Option<HashMap<&'static str, Cow<'static, str>>>,
    pub base: El,
    pub extension: Ext,
}

pub(crate) type SerializeEventFn = dyn FnOnce() -> Result<(u32, String), InternalError>;

pub fn default<El: Default, Ext: Default>() -> Attributes<El, Ext> {
    Attributes::default()
}

impl<El, Ext> Attributes<El, Ext> {
    pub fn custom(
        mut self,
        name: &'static str,
        value: impl Into<Cow<'static, str>>,
    ) -> Attributes<El, Ext> {
        // TODO: replace with `get_or_insert_default();` once stable
        let attrs = match self.custom.as_mut() {
            Some(attrs) => attrs,
            None => {
                self.custom = Some(Default::default());
                self.custom.as_mut().unwrap()
            }
        };
        attrs.insert(name, value.into());
        self
    }

    /// Unique identifier across the document.
    pub fn id(mut self, id: impl Into<Cow<'static, str>>) -> Attributes<El, Ext> {
        self.id = Some(id.into());
        self
    }

    /// The various classes that the element belongs to.
    pub fn class(mut self, class: impl Into<Cow<'static, str>>) -> Attributes<El, Ext> {
        self.class = Some(class.into());
        self
    }

    /// Append classes that the element belongs to.
    pub fn add_class(mut self, class: impl Into<Cow<'static, str>>) -> Attributes<El, Ext> {
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

impl<El, Ext> ElementExt for Attributes<El, Ext>
where
    El: ElementExt,
    Ext: ElementExt,
{
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        let Attributes {
            id,
            class,
            on_click,
            global,
            aria,
            custom,
            base,
            extension,
        } = self;

        if let Some(event) = on_click {
            // TODO: directly write into el?
            let (id, payload) = &(event)()?;
            r.attribute("cabin-click", id)
                .map_err(crate::error::InternalError::from)?;
            r.attribute("cabin-click-payload", payload)
                .map_err(crate::error::InternalError::from)?;
        }

        if let Some(id) = id {
            r.attribute("id", id)
                .map_err(crate::error::InternalError::from)?;
        }

        if let Some(class) = class {
            r.attribute("class", class)
                .map_err(crate::error::InternalError::from)?;
        }

        if let Some(attrs) = custom {
            for (name, value) in attrs {
                if !valid_attribute_name(name) {
                    return Err(InternalError::InvalidAttributeName {
                        name: name.to_string(),
                    }
                    .into());
                }
                r.attribute(name, value)
                    .map_err(crate::error::InternalError::from)?;
            }
        }

        if let Some(global) = global {
            global.render(r)?;
        }
        if let Some(aria) = aria {
            aria.render(r)?;
        }
        base.render(r)?;
        extension.render(r)?;

        Ok(())
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

impl<El: Default, Ext: Default> From<()> for Attributes<El, Ext> {
    fn from(_: ()) -> Self {
        Attributes::default()
    }
}
