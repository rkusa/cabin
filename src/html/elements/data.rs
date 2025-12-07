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
    pub fn data(&self) -> VoidElement<marker::Data> {
        VoidElement::new(self.acquire_renderer(), "data")
    }
}

pub mod marker {
    pub struct Data;
}

impl Data for VoidElement<marker::Data> {}
impl Common for VoidElement<marker::Data> {}
impl Global for VoidElement<marker::Data> {}

/// The `data` element represents a group of one or more columns in the [super::table] that is its
/// parent, if it has a parent and that is a [super::table] element.
pub trait Data: WithAttribute {
    /// Machine-readable value.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Value(value.into()))
    }
}
