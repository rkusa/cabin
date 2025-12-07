use std::borrow::Cow;

use cabin_macros::Attribute;

use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `slot` element defines a slot. It is typically used in a shadow tree. A `slot` element
    /// represents its assigned nodes, if any, and its contents otherwise.
    pub fn slot(&self) -> Element<marker::Slot> {
        Element::new(self.acquire_renderer(), "slot")
    }
}

pub mod marker {
    pub struct Slot;
}

impl Slot for Element<marker::Slot> {}
impl Global for Element<marker::Slot> {}

/// The `slot` element defines a slot. It is typically used in a shadow tree. A `slot` element
/// represents its assigned nodes, if any, and its contents otherwise.
pub trait Slot: WithAttribute {
    /// Name of shadow tree slot.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }
}

/// Name of shadow tree slot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);
