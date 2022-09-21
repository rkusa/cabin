use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;

use crate::render::{ElementRenderer, Renderer};

pub struct Attribute<N> {
    name: &'static str,
    value: Cow<'static, str>,
    next: N,
}

pub trait Attributes {
    fn render<'a>(&self, r: &mut ElementRenderer<'a>) -> Result<(), fmt::Error>;
}

impl Attributes for () {
    fn render<'a>(&self, _r: &mut ElementRenderer<'a>) -> Result<(), fmt::Error> {
        Ok(())
    }
}

impl<N> Attribute<N> {
    pub fn new(name: &'static str, value: impl Into<Cow<'static, str>>, next: N) -> Self {
        Self {
            name,
            value: value.into(),
            next,
        }
    }
}

impl<N> Attributes for Attribute<N>
where
    N: Attributes,
{
    fn render<'a>(&self, r: &mut ElementRenderer<'a>) -> Result<(), fmt::Error> {
        r.attribute(self.name, &self.value)?;
        self.next.render(r)
    }
}
