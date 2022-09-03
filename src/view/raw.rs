use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;

use twox_hash::XxHash32;

use crate::View;

use super::ViewHash;

pub struct Raw<'a>(pub Cow<'a, str>);

pub fn raw<'a>(raw: impl Into<Cow<'a, str>>) -> Raw<'a> {
    Raw(raw.into())
}

impl<'a, A> View<A> for Raw<'a> {
    fn render(self, mut out: impl Write) -> Result<ViewHash, fmt::Error> {
        let mut hasher = XxHash32::default();
        hasher.write(self.0.as_bytes());
        let hash = hasher.finish() as u32;
        // TODO: safe escape HTML
        write!(out, "<!-- {} -->{}", hash, self.0)?;
        Ok(ViewHash::Leaf(hash))
    }
}
