use std::borrow::Cow;

use cabin_macros::Attribute;

use super::global::Global;
use crate::View;
use crate::html::Html;
use crate::html::attributes::{Attributes, WithAttribute};

/// The `slot` element defines a slot. It is typically used in a shadow tree. A `slot` element
/// represents its assigned nodes, if any, and its contents otherwise.
pub fn slot(content: impl View) -> Html<marker::Slot, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("slot", (), content)
}

pub mod marker {
    pub struct Slot;
}

impl<A: Attributes, V: 'static> Slot for Html<marker::Slot, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Slot, A, V> {}

/// The `slot` element defines a slot. It is typically used in a shadow tree. A `slot` element
/// represents its assigned nodes, if any, and its contents otherwise.
pub trait Slot: WithAttribute {
    /// Name of shadow tree slot.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }
}

/// Name of shadow tree slot.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);
