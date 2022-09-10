use std::fmt::{self, Write};

use super::{HashTree, IntoView};
use crate::{Render, View};

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
pub struct Text<F> {
    write: F,
}

impl<F: Fn(&mut dyn fmt::Write) -> fmt::Result> Text<F> {
    pub fn new(write: F) -> Self {
        Text { write }
    }
}

impl<S, F> View<S> for Text<F>
where
    F: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    type Render = TextRenderer<F>;

    fn prepare(self, hash_tree: &mut HashTree) -> Option<Self::Render> {
        let mut node = hash_tree.node();
        (self.write)(&mut HashFmt(&mut node)).unwrap(); // TODO: unwrap
        let hash = node.end();
        hash_tree.changed_or_else(hash, || TextRenderer(self.write))
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

pub struct TextRenderer<F>(F);

impl<F> Render for TextRenderer<F>
where
    F: Fn(&mut dyn fmt::Write) -> fmt::Result,
{
    fn render(&self, mut out: &mut dyn Write, _is_update: bool) -> fmt::Result {
        (self.0)(&mut out)
    }
}

struct HashFmt<'a>(&'a mut dyn std::hash::Hasher);

impl<'a> fmt::Write for HashFmt<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}
