use std::borrow::Cow;

use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::attribute::WithAttribute;
use crate::void_element::{VoidElement, VoidElementProxy};

/// The `data` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub fn data() -> VoidElement<marker::Data> {
    VoidElement::new("data")
}

pub mod marker {
    pub struct Data;
}

impl<P> Data<marker::Data> for P where P: VoidElementProxy<marker::Data> {}
impl<P> Common<marker::Data> for P where P: VoidElementProxy<marker::Data> {}
impl<P> Global<marker::Data> for P where P: VoidElementProxy<marker::Data> {}

/// The `data` element represents a group of one or more columns in the [super::table] that is its
/// parent, if it has a parent and that is a [super::table] element.
pub trait Data<T>: WithAttribute {
    /// Machine-readable value.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Value(value.into()))
    }
}
