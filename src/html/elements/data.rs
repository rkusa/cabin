use std::borrow::Cow;

use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::void_element::VoidElement;

impl Context {
    /// The `data` element represents a group of one or more columns in the [super::table] that is
    /// its parent, if it has a parent and that is a [super::table] element.
    pub fn data(&self) -> VoidElement<'_, marker::Data> {
        VoidElement::new(self, "data")
    }
}

pub mod marker {
    pub struct Data;
}

impl<'v> Data for VoidElement<'v, marker::Data> {}
impl<'v> Common for VoidElement<'v, marker::Data> {}
impl<'v> Global for VoidElement<'v, marker::Data> {}

/// The `data` element represents a group of one or more columns in the [super::table] that is its
/// parent, if it has a parent and that is a [super::table] element.
pub trait Data: WithAttribute {
    /// Machine-readable value.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Value(value.into()))
    }
}
