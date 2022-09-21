use std::fmt;

use super::{IntoView, View};
use crate::render::Renderer;

#[macro_export]
macro_rules! text {
    ($fmt:expr) => {
        ::crabweb::Text::new(
            move |wr: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {
                wr.write_fmt(format_args!($fmt))
            },
        )
    };
    ($fmt:expr, $($args:tt)*) => {
        ::crabweb::Text::new(
            move |wr: &mut dyn ::std::fmt::Write| -> ::std::fmt::Result {
                wr.write_fmt(format_args!($fmt, $($args)*))
            },
        )
    };
}

pub use text;

// Note: Cannot directly implement View for std::fmt::Arguments due to resulting lifetime issues.
pub struct Text<F>(F);

impl<F: Fn(&mut dyn fmt::Write) -> fmt::Result> Text<F> {
    pub fn new(write: F) -> Self {
        Text(write)
    }
}

impl<S, F> View<S> for Text<F>
where
    F: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        (self.0)(r)
    }
}

impl<S, F> IntoView<Text<F>, S> for Text<F>
where
    F: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn into_view(self) -> Text<F> {
        self
    }
}

struct HashFmt<'a>(&'a mut dyn std::hash::Hasher);

impl<'a> fmt::Write for HashFmt<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}
