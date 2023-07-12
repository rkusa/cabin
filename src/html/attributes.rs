use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;

use super::elements::aria::{Aria, AriaAttributes};
use super::elements::common::{Common, CommonAttributes};
use super::elements::global::{Global, GlobalAttributes};
use super::elements::ElementExt;
use crate::error::InternalError;
use crate::render::ElementRenderer;

pub trait Attributes2: Sized + 'static {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error>;

    fn get<T: 'static>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref(self)
    }

    fn with<A: Attributes2>(self, attr: A) -> Pair<A, Self> {
        Pair::new(attr, self)
    }
}

pub struct Pair<L, R> {
    left: L,
    right: R,
}

impl<L, R> Pair<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Pair { left, right }
    }
}

impl Attributes2 for () {
    fn render(self, _r: &mut ElementRenderer) -> Result<(), crate::Error> {
        Ok(())
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        None
    }
}

impl<L: Attributes2, R: Attributes2> Attributes2 for Pair<L, R> {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        self.left.render(r)?;
        self.right.render(r)?;
        Ok(())
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        self.left.get().or_else(|| self.right.get())
    }
}

impl Attributes2 for (&'static str, Cow<'static, str>) {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        r.attribute(self.0, self.1).map_err(InternalError::from)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Attributes<El> {
    pub common: CommonAttributes,
    // Boxed to not blow up struct size.
    pub global: Option<Box<GlobalAttributes>>,
    pub aria: Option<Box<AriaAttributes>>,
    pub custom: Option<HashMap<&'static str, Cow<'static, str>>>,
    pub base: El,
}

pub fn default<El: Default>() -> Attributes<El> {
    Attributes::default()
}

impl<El> Attributes<El> {
    pub fn custom(
        mut self,
        name: &'static str,
        value: impl Into<Cow<'static, str>>,
    ) -> Attributes<El> {
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

    /// Append classes that the element belongs to.
    pub fn add_class(mut self, class: impl Into<Cow<'static, str>>) -> Attributes<El> {
        self.common.class = match self.common.class {
            Some(before) => Some(format!("{} {}", before, class.into()).into()),
            None => Some(class.into()),
        };
        self
    }
}

impl<El> ElementExt for Attributes<El>
where
    El: ElementExt,
{
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        let Attributes {
            common,
            global,
            aria,
            custom,
            base,
        } = self;

        common.render(r)?;
        if let Some(global) = global {
            global.render(r)?;
        }
        if let Some(aria) = aria {
            aria.render(r)?;
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

        base.render(r)?;

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

impl<El: Default> From<()> for Attributes<El> {
    fn from(_: ()) -> Self {
        Attributes::default()
    }
}

impl<El> AsMut<CommonAttributes> for Attributes<El> {
    fn as_mut(&mut self) -> &mut CommonAttributes {
        &mut self.common
    }
}

impl<El> Common for Attributes<El> {}

impl<El> AsMut<GlobalAttributes> for Attributes<El> {
    fn as_mut(&mut self) -> &mut GlobalAttributes {
        // TODO: use get_or_insert_default() once stable
        if self.global.is_none() {
            self.global = Default::default();
        }
        self.global.as_mut().unwrap()
    }
}

impl<El> Global for Attributes<El> {}

impl<El> AsMut<AriaAttributes> for Attributes<El> {
    fn as_mut(&mut self) -> &mut AriaAttributes {
        // TODO: use get_or_insert_default() once stable
        if self.aria.is_none() {
            self.aria = Default::default();
        }
        self.aria.as_mut().unwrap()
    }
}

impl<El> Aria for Attributes<El> {}
