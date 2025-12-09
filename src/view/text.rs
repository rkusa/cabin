use std::fmt::Arguments;

use crate::View;
use crate::render::Renderer;

#[macro_export]
macro_rules! text {
    ($fmt:expr) => {
        $crate::view::text::Text::new(
            move |r: &mut $crate::render::Renderer|
                -> Result<(), $crate::error::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(
                    &mut $crate::render::Escape::content(&mut txt),
                    format_args!($fmt)
                ).map_err($crate::error::InternalError::from)?;
                txt.end();
                Ok(())
            },
        )
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::view::text::Text::new(
            move |r: &mut $crate::render::Renderer|
                -> Result<(), $crate::error::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(
                    &mut $crate::render::Escape::content(&mut txt),
                    format_args!($fmt, $($args)*)
                ).map_err($crate::error::InternalError::from)?;
                txt.end();
                Ok(())
            },
        )
    };
}

pub use text;

pub struct Text<F>(F);

impl<F> Text<F>
where
    F: FnOnce(&mut Renderer) -> Result<(), crate::Error>,
{
    pub fn new(write: F) -> Self {
        Text(write)
    }
}

impl<F> View for Text<F>
where
    F: FnOnce(&mut Renderer) -> Result<(), crate::Error>,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        (self.0)(r)?;
        Ok(())
    }
}

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
