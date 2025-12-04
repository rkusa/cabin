use std::borrow::Cow;

use super::aria::Aria;
use super::button::Disabled;
use super::common::Common;
use super::global::Global;
use super::option::Label;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `optgroup` element represents a group of [super::option] elements with a common label.
    pub fn optgroup(&self) -> Element<'_, marker::OptGroup> {
        Element::new(self, "optgroup")
    }
}

pub mod marker {
    pub struct OptGroup;
}

impl<'v> OptGroup for Element<'v, marker::OptGroup> {}
impl<'v> Common for Element<'v, marker::OptGroup> {}
impl<'v> Global for Element<'v, marker::OptGroup> {}
impl<'v> Aria for Element<'v, marker::OptGroup> {}

/// The `optgroup` element represents a group of [super::option] elements with a common label.
pub trait OptGroup: WithAttribute {
    /// Whether the form control is disabled.
    fn disabled(self) -> Self {
        self.with_disabled(true)
    }

    /// Whether the form control is disabled.
    fn with_disabled(self, disabled: bool) -> Self {
        self.with_attribute(Disabled(disabled))
    }

    /// User-visible label.
    fn label(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Label(value.into()))
    }
}
