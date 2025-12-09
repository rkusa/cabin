use std::any::Any;
use std::borrow::Cow;

use crate::Context;
use crate::render::Renderer;
use crate::view::internal::Internal;

pub trait Attribute {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error>;
    fn as_any(&self) -> Option<&dyn Any>;

    fn get<T: 'static>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref(self.as_any()?)
    }

    // fn get_mut<T: 'static>(&mut self) -> Option<&mut T>
    // where
    //     Self: Sized + 'static,
    // {
    //     <dyn Any>::downcast_mut(self)
    // }
}

pub trait WithAttribute: Sized {
    fn with_attribute(self, attr: impl Attribute) -> Self;
}

impl Attribute for () {
    fn render(self, _r: &mut Renderer) -> Result<(), crate::Error> {
        Ok(())
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self as &dyn Any)
    }
}

impl<'v> Attribute for (&'v str, Cow<'v, str>) {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.attribute(self.0, self.1);
        Ok(())
    }

    fn as_any(&self) -> Option<&dyn Any> {
        None
    }
}

pub struct Attributes(Internal<AttributesBuilder>);

pub struct AttributesBuilder {
    renderer: Renderer,
}

impl Attributes {
    pub fn new() -> Self {
        Attributes(Internal::new(AttributesBuilder {
            renderer: Context::acquire_renderer_from_task(),
        }))
    }
}

impl WithAttribute for Attributes {
    fn with_attribute(mut self, attr: impl Attribute) -> Self {
        let Some(builder) = self.0.builder_mut() else {
            return self;
        };

        if let Err(err) = attr.render(&mut builder.renderer) {
            self.0.errored(err);
        }

        self
    }
}

impl Attribute for Attributes {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        let builder = self.0.take_builder()?;
        r.append(builder.renderer);
        Ok(())
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self as &dyn Any)
    }
}
