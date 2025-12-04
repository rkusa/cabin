use std::borrow::Cow;

use crate::render::Renderer;

pub trait Attribute {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error>;
}

pub trait WithAttribute: Sized {
    fn with_attribute(self, attr: impl Attribute) -> Self;
}

impl Attribute for () {
    fn render(self, _r: &mut Renderer) -> Result<(), crate::Error> {
        Ok(())
    }
}

impl<'v> Attribute for (&'v str, Cow<'v, str>) {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.attribute(self.0, self.1);
        Ok(())
    }
}
