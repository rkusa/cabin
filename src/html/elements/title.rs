use std::borrow::Cow;

use http::{HeaderName, HeaderValue};

use super::global::Global;
use crate::View;
use crate::attribute::{Attribute, WithAttribute};
use crate::context::Context;
use crate::element::{Element, ElementContent};
use crate::render::Renderer;
use crate::view::RenderFuture;

impl Context {
    /// The `title` element represents the document's title or name. Authors should use titles that
    /// identify their documents even when they are used out of context, for example in a user's
    /// history or bookmarks, or in search results. The document's title is often different from
    /// its first heading, since the first heading does not have to stand alone when taken out
    /// of context.
    pub fn title(&self) -> TitleElement<'_> {
        TitleElement(Element::new(self, "title"))
    }
}

pub struct TitleElement<'v>(Element<'v, marker::Title>);
pub struct TitleContent<'v>(ElementContent<'v>);

mod marker {
    pub struct Title;
}

impl<'v> TitleElement<'v> {
    pub fn child(self, child: impl Into<Cow<'v, str>>) -> TitleContent<'v> {
        TitleContent(self.0.child(child.into()))
    }
}

impl<'v> TitleContent<'v> {
    pub fn child(self, child: impl Into<Cow<'v, str>>) -> Self {
        Self(self.0.child(child.into()))
    }
}

impl<'v> View<'v> for TitleElement<'v> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render(c, r)
    }
}

impl<'v> View<'v> for TitleContent<'v> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render(c, r)
    }
}

impl<'v> WithAttribute for TitleElement<'v> {
    fn with_attribute(self, attr: impl Attribute) -> Self {
        Self(self.0.with_attribute(attr))
    }
}

impl<'v> Global for TitleElement<'v> {}

pub struct TitleUpdate(pub Cow<'static, str>);

pub fn title_update(title: impl Into<Cow<'static, str>>) -> TitleUpdate {
    TitleUpdate(title.into())
}

impl<'v> View<'v> for TitleUpdate {
    fn render(self, _c: &'v Context, mut r: Renderer) -> RenderFuture<'v> {
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

        RenderFuture::ready(Ok(r))
    }
}
