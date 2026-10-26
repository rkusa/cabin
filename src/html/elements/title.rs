use std::borrow::Cow;

use http::HeaderName;

use crate::View;
use crate::header::percent_encoded;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::render::Renderer;
use crate::view::RenderFuture;

/// The `title` element represents the document's title or name. Authors should use titles that
/// identify their documents even when they are used out of context, for example in a user's history
/// or bookmarks, or in search results. The document's title is often different from its first
/// heading, since the first heading does not have to stand alone when taken out of context.
pub fn title(title: impl Into<Cow<'static, str>>) -> Html<marker::Title, ()> {
    Html::new("title", (), title.into())
}

pub mod marker {
    pub struct Title;
}

impl<A: Attributes> Global for Html<marker::Title, A> {}

pub struct TitleUpdate(pub Cow<'static, str>);

pub fn title_update(title: impl Into<Cow<'static, str>>) -> TitleUpdate {
    TitleUpdate(title.into())
}

impl View for TitleUpdate {
    fn render(self, mut r: Renderer) -> RenderFuture {
        if r.is_update() {
            r.headers_mut().insert(
                HeaderName::from_static("x-cabin-title"),
                percent_encoded(&self.0),
            );
        }

        RenderFuture::Ready(Ok(r))
    }
}
