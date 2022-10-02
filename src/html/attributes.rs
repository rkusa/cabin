use std::borrow::Cow;
use std::fmt;

use crate::render::ElementRenderer;

pub struct Attribute<'a, N> {
    name: &'static str,
    value: Cow<'a, str>,
    next: N,
}

pub trait Attributes {
    fn render(&self, r: &mut ElementRenderer) -> Result<(), fmt::Error>;
}

impl Attributes for () {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

impl<'a, N> Attribute<'a, N> {
    pub fn new(name: &'static str, value: impl Into<Cow<'a, str>>, next: N) -> Self {
        Self {
            name,
            value: value.into(),
            next,
        }
    }
}

impl<'a, N> Attributes for Attribute<'a, N>
where
    N: Attributes,
{
    fn render(&self, r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        r.attribute(self.name, &self.value)?;
        self.next.render(r)
    }
}
