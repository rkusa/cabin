use std::borrow::Cow;

use cabin_macros::Attribute;

use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `slot` element defines a slot. It is typically used in a shadow tree. A `slot` element
/// represents its assigned nodes, if any, and its contents otherwise.
pub fn slot() -> Element<marker::Slot> {
    Element::new("slot")
}

pub mod marker {
    pub struct Slot;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Slot> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Slot<(marker::Slot, P)> for E where E: ElementProxy<marker::Slot, P> {}
impl<E, P> Global<(marker::Slot, P)> for E where E: ElementProxy<marker::Slot, P> {}

/// The `slot` element defines a slot. It is typically used in a shadow tree. A `slot` element
/// represents its assigned nodes, if any, and its contents otherwise.
pub trait Slot<T>: WithAttribute {
    /// Name of shadow tree slot.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }
}

/// Name of shadow tree slot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);
