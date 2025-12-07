use std::borrow::Cow;

use http::{HeaderName, HeaderValue};

use super::global::Global;
use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::element::{Element, ElementContent};
use crate::render::Renderer;

/// The `title` element represents the document's title or name. Authors should use titles that
/// identify their documents even when they are used out of context, for example in a user's
/// history or bookmarks, or in search results. The document's title is often different from
/// its first heading, since the first heading does not have to stand alone when taken out
/// of context.
pub fn title() -> TitleElement {
    TitleElement(Element::new("title"))
}

pub struct TitleElement(Element<marker::Title>);
pub struct TitleContent(ElementContent);

mod marker {
    pub struct Title;
}

impl TitleElement {
    pub fn child<'s>(self, child: impl Into<Cow<'s, str>>) -> TitleContent {
        TitleContent(self.0.child(child.into()))
    }
}

impl TitleContent {
    pub fn child<'s>(self, child: impl Into<Cow<'s, str>>) -> Self {
        Self(self.0.child(child.into()))
    }
}

impl View for TitleElement {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        View::render(self.0, r)
    }
}

impl View for TitleContent {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        self.0.render(r)
    }
}

impl WithAttribute for TitleElement {
    fn with_attribute(self, attr: impl Attribute) -> Self {
        Self(self.0.with_attribute(attr))
    }
}

impl Global for TitleElement {}

pub struct TitleUpdate(pub Cow<'static, str>);

pub fn title_update(title: impl Into<Cow<'static, str>>) -> TitleUpdate {
    TitleUpdate(title.into())
}

impl View for TitleUpdate {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        if r.is_update() {
            match HeaderValue::from_str(&self.0) {
                Ok(v) => {
                    let h = r.headers_mut();
                    h.insert(HeaderName::from_static("x-cabin-title"), v);
                }
                Err(err) => {
                    tracing::error!(%err, "invalid header value for X-CABIN-TITLE");
                }
            }
        }

        Ok(())
    }
}
