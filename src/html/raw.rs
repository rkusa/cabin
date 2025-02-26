use std::borrow::Cow;
use std::fmt::Write;

use crate::View;
use crate::render::Renderer;
use crate::view::RenderFuture;

pub struct Raw(pub Cow<'static, str>);

pub fn raw(txt: impl Into<Cow<'static, str>>) -> Raw {
    Raw(txt.into())
}

impl View for Raw {
    fn render(self, r: Renderer, _include_hash: bool) -> RenderFuture {
        let mut txt = r.text();
        RenderFuture::ready(
            txt.write_str(&self.0)
                .map_err(crate::error::InternalError::from)
                .map_err(crate::error::Error::from)
                .and_then(|_| txt.end()),
        )
    }
}
