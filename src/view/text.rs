use super::{RenderFuture, View};
use crate::context::Context;
use crate::render::Renderer;

#[macro_export]
macro_rules! text {
    ($fmt:expr) => {
        $crate::view::text::Text::new(
            move |r: $crate::render::Renderer|
                -> Result<$crate::render::Renderer, $crate::error::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(
                    &mut $crate::render::Escape::content(&mut txt),
                    format_args!($fmt)
                ).map_err($crate::error::InternalError::from)?;
                Ok(txt.end())
            },
        )
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::view::text::Text::new(
            move |r: $crate::render::Renderer|
                -> Result<$crate::render::Renderer, $crate::error::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(
                    &mut $crate::render::Escape::content(&mut txt),
                    format_args!($fmt, $($args)*)
                ).map_err($crate::error::InternalError::from)?;
                Ok(txt.end())
            },
        )
    };
}

pub use text;

// Note: Cannot directly implement View for std::fmt::Arguments due to resulting lifetime issues.
pub struct Text<F>(F);

impl<F> Text<F>
where
    F: FnOnce(Renderer) -> Result<Renderer, crate::Error>,
{
    pub fn new(write: F) -> Self {
        Text(write)
    }
}

impl<'v, F> View<'v> for Text<F>
where
    F: FnOnce(Renderer) -> Result<Renderer, crate::Error> + 'v,
{
    fn render(self, _c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        RenderFuture::ready((self.0)(r))
    }
}
