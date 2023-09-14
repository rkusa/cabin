use std::borrow::Cow;

use http::{HeaderName, HeaderValue};

use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::render::Renderer;
use crate::view::RenderFuture;
use crate::View;

/// The `title` element represents the document's title or name. Authors should use titles that
/// identify their documents even when they are used out of context, for example in a user's history
/// or bookmarks, or in search results. The document's title is often different from its first
/// heading, since the first heading does not have to stand alone when taken out of context.
pub fn title(title: impl Into<Cow<'static, str>>) -> Html<marker::Title, (), Cow<'static, str>> {
    Html::new("title", (), title.into())
}

pub mod marker {
    pub struct Title;
}

impl<A: Attributes, V: 'static> Global for Html<marker::Title, A, V> {}

pub struct TitleUpdate(pub Cow<'static, str>);

pub fn title_update(title: impl Into<Cow<'static, str>>) -> TitleUpdate {
    TitleUpdate(title.into())
}

impl View for TitleUpdate {
    fn render(self, mut r: Renderer, _include_hash: bool) -> RenderFuture {
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

        RenderFuture::Ready(Some(Ok(r)))
    }

    fn prime(&mut self) {
        self.0.prime()
    }
}
