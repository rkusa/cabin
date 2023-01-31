use std::borrow::Cow;

use crate::html::attributes::Attributes;
use crate::html::Html;
use crate::render::ElementRenderer;

#[derive(Default)]
pub struct Anchor {
    href: Option<Cow<'static, str>>,
}

impl<V, A> Html<V, A, Anchor> {
    pub fn href(mut self, href: impl Into<Cow<'static, str>>) -> Self {
        self.kind.href = Some(href.into());
        self
    }
}

impl Attributes for Anchor {
    fn render(&self, r: &mut ElementRenderer) -> Result<(), std::fmt::Error> {
        if let Some(href) = &self.href {
            r.attribute("href", href)?;
        }

        Ok(())
    }
}
