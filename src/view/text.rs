use std::fmt::Arguments;
pub use std::format_args as text;

use crate::View;
use crate::render::Renderer;

impl<'a> View for Arguments<'a> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        let mut txt = r.text();
        if let Err(err) =
            ::std::fmt::Write::write_fmt(&mut crate::render::Escape::content(&mut txt), self)
        {
            return Err(crate::error::InternalError::from(err).into());
        }
        txt.end();
        Ok(())
    }
}
