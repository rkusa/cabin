use std::borrow::Cow;

use http::{HeaderName, HeaderValue};

use super::global::Global;
use crate::View;
use crate::element::{Element, ElementProxy};
use crate::render::Renderer;

/// The `title` element represents the document's title or name. Authors should use titles that
/// identify their documents even when they are used out of context, for example in a user's
/// history or bookmarks, or in search results. The document's title is often different from
/// its first heading, since the first heading does not have to stand alone when taken out
/// of context.
pub fn title() -> Element<marker::Title> {
    Element::new("title")
}

pub mod marker {
    pub struct Title;

    impl<'v, S: Into<std::borrow::Cow<'v, str>>> crate::element::IntoChild<'v, Title> for S {
        fn into_child(self) -> impl crate::View + 'v {
            self.into()
        }
    }
}

impl<E, P> Global<(marker::Title, P)> for E where E: ElementProxy<marker::Title, P> {}

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
