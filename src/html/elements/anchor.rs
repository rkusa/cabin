use std::borrow::Cow;

use crate::html::attributes::Attributes;
use crate::html::Html;
use crate::render::ElementRenderer;
use crate::View;

#[derive(Default)]
pub struct Anchor {
    href: Option<Cow<'static, str>>,
}

impl<V, A> Html<V, A, Anchor>
where
    V: View,
{
    pub fn href(mut self, href: impl Into<Cow<'static, str>>) -> Self {
        self.kind.href = Some(href.into());
        self
    }
}

impl Attributes for Anchor {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        if let Some(href) = &self.href {
            r.attribute("href", href)
                .map_err(crate::error::InternalError::from)?;
        }

        Ok(())
    }
}
