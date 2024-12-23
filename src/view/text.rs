use super::{RenderFuture, View};
use crate::render::Renderer;

#[macro_export]
macro_rules! text {
    ($fmt:expr) => {
        $crate::html::Text::new(
            move |r: $crate::render::Renderer|
                -> Result<$crate::render::Renderer, $crate::error::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(
                    &mut $crate::render::Escape::content(&mut txt),
                    format_args!($fmt)
                ).map_err($crate::error::InternalError::from)?;
                txt.end()
            },
        )
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::html::Text::new(
            move |r: $crate::render::Renderer|
                -> Result<$crate::render::Renderer, $crate::error::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(
                    &mut $crate::render::Escape::content(&mut txt),
                    format_args!($fmt, $($args)*)
                ).map_err($crate::error::InternalError::from)?;
                txt.end()
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

impl<F> View for Text<F>
where
    F: FnOnce(Renderer) -> Result<Renderer, crate::Error> + Send + 'static,
{
    fn render(self, r: Renderer, _include_hash: bool) -> RenderFuture {
        RenderFuture::ready((self.0)(r))
    }
}
