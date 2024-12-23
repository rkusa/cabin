use std::any::Any;
use std::borrow::Cow;

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
            Pair::with_fake(self)
        } else {
            Pair::new(attr, self)
        }
    }
}

pub struct Pair<L, R> {
    left: Option<L>,
    right: Option<R>,
}

impl<L, R> Pair<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Pair {
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn with_fake(right: R) -> Self {
        Pair {
            left: None,
            right: Some(right),
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

impl<L: Attributes, R: Attributes> Attributes for Pair<L, R> {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        if let Some(left) = self.left {
            left.render(r)?
        };
        if let Some(right) = self.right {
            right.render(r)?
        };
        Ok(())
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        self.left
            .as_ref()
            .and_then(|l| l.get())
            .or_else(|| self.right.as_ref().and_then(|r| r.get()))
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.left
            .as_mut()
            .and_then(|l| l.get_mut())
            .or_else(|| self.right.as_mut().and_then(|r| r.get_mut()))
    }
}

impl Attributes for (&'static str, Cow<'static, str>) {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        r.attribute(self.0, self.1)
            .map_err(crate::error::InternalError::from)?;
        Ok(())
    }

    fn with<A: Attributes>(self, attr: A) -> Pair<A, Self> {
        Pair::new(attr, self)
    }
}
