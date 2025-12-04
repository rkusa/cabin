use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// A `label` element that represents a caption that can be associated with a specific form
    /// control.
    pub fn label(&self) -> Element<'_, marker::Label> {
        Element::new(self, "label")
    }
}

pub mod marker {
    pub struct Label;
}

impl<'v> Label for Element<'v, marker::Label> {}
impl<'v> Common for Element<'v, marker::Label> {}
impl<'v> Global for Element<'v, marker::Label> {}
impl<'v> Aria for Element<'v, marker::Label> {}

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
pub trait Label: WithAttribute {
    /// The id of the form control the label is the caption for.
    fn r#for(self, r#for: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(For(r#for.into()))
    }
}

/// The id of the form control the label is the caption for.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct For(pub Cow<'static, str>);
