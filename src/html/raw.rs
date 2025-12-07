use std::borrow::Cow;
use std::fmt::Write;

use crate::View;
use crate::render::Renderer;

pub struct Raw<'s>(pub Cow<'s, str>);

pub fn raw<'s>(txt: impl Into<Cow<'s, str>>) -> Raw<'s> {
    Raw(txt.into())
}

impl<'s> View for Raw<'s> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        let mut txt = r.text();
        txt.write_str(&self.0)
            .map_err(crate::error::InternalError::from)?;
        txt.end();
        Ok(())
    }
}
