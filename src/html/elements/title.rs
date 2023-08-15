use std::borrow::Cow;

use http::{HeaderName, HeaderValue};

use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::render::Renderer;
use crate::view::RenderFuture;
use crate::View;

/// TODO
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
