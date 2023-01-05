use std::fmt;
use std::future::{ready, Ready};

use super::View;
use crate::render::Renderer;

#[macro_export]
macro_rules! text {
    ($fmt:expr) => {
        ::rustend::html::Text::new(
            move |r: ::rustend::Renderer|
                -> Result<::rustend::Renderer, ::std::fmt::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(&mut txt, format_args!($fmt))?;
                txt.end()
            },
        )
    };
    ($fmt:expr, $($args:tt)*) => {
        ::rustend::html::Text::new(
            move |r: ::rustend::Renderer|
                -> Result<::rustend::Renderer, ::std::fmt::Error>
            {
                let mut txt = r.text();
                ::std::fmt::Write::write_fmt(&mut txt, format_args!($fmt, $($args)*))?;
                txt.end()
            },
        )
    };
}

pub use text;

// Note: Cannot directly implement View for std::fmt::Arguments due to resulting lifetime issues.
pub struct Text<F>(F);

impl<F: Fn(Renderer) -> Result<Renderer, fmt::Error>> Text<F> {
    pub fn new(write: F) -> Self {
        Text(write)
    }
}

impl<S, F> View<S> for Text<F>
where
    F: Fn(Renderer) -> Result<Renderer, fmt::Error>,
{
    type Future = Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        ready((self.0)(r))
    }
}

struct HashFmt<'a>(&'a mut dyn std::hash::Hasher);

impl<'a> fmt::Write for HashFmt<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}