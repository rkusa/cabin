use std::fmt;

use super::View;
use crate::render::Renderer;

#[macro_export]
macro_rules! text {
    ($fmt:expr) => {
        $crate::html::Text::new(
            move |r: $crate::render::Renderer|
                -> Result<$crate::render::Renderer, $crate::error::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(&mut txt, format_args!($fmt)).map_err($crate::error::InternalError::from)?;
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
                ::std::fmt::Write::write_fmt(&mut txt, format_args!($fmt, $($args)*)).map_err($crate::error::InternalError::from)?;
                txt.end()
            },
        )
    };
}

pub use text;

// Note: Cannot directly implement View for std::fmt::Arguments due to resulting lifetime issues.
pub struct Text<F>(F);

impl<F: Fn(Renderer) -> Result<Renderer, crate::Error>> Text<F> {
    pub fn new(write: F) -> Self {
        Text(write)
    }
}

impl<F> View for Text<F>
where
    F: Fn(Renderer) -> Result<Renderer, crate::Error>,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        (self.0)(r)
    }
}

struct HashFmt<'a>(&'a mut dyn std::hash::Hasher);

impl<'a> fmt::Write for HashFmt<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}
