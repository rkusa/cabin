use std::any::Any;
use std::borrow::Cow;

pub use crate::pair::Pair;
use crate::render::ElementRenderer;

pub trait WithAttribute: Sized {
    type Output<T>
    where
        T: Attributes;
    fn with_attribute<T: Attributes>(self, attr: T) -> Self::Output<T>;

    fn get_attribute<T: 'static>(&self) -> Option<&T>;

    fn get_attribute_mut<T: 'static>(&mut self) -> Option<&mut T>;
}

pub trait Attributes: Sized + Send + 'static {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error>;

    fn get<T: 'static>(&self) -> Option<&T> {
        <dyn Any>::downcast_ref(self)
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        <dyn Any>::downcast_mut(self)
    }

    fn with<A: Attributes>(mut self, attr: A) -> Pair<A, Self> {
        // replace if already exists
        if let Some(val) = self.get_mut::<A>() {
            *val = attr;
            Pair::right(self)
        } else {
            Pair::new(attr, self)
        }
    }
}

impl Attributes for () {
    fn render(self, _r: &mut ElementRenderer) -> Result<(), crate::Error> {
        Ok(())
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        None
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        None
    }
}

impl Attributes for (&'static str, Cow<'static, str>) {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        r.attribute(self.0, self.1);
        Ok(())
    }

    fn with<A: Attributes>(self, attr: A) -> Pair<A, Self> {
        Pair::new(attr, self)
    }
}
