use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
pub fn label() -> Element<marker::Label> {
    Element::new("label")
}

pub mod marker {
    pub struct Label;
}

impl Label for Element<marker::Label> {}
impl Common for Element<marker::Label> {}
impl Global for Element<marker::Label> {}
impl Aria for Element<marker::Label> {}

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
