use std::borrow::Cow;

use super::button::Disabled;
use super::common::Common;
use super::global::Global;
use super::option::Label;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `optgroup` element represents a group of [super::option] elements with a common label.
pub fn optgroup(content: impl View) -> Html<marker::OptGroup, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("optgroup", (), content)
}

pub mod marker {
    pub struct OptGroup;
}

impl<A: Attributes> OptGroup for Html<marker::OptGroup, A> {}
impl<A: Attributes> Common for Html<marker::OptGroup, A> {}
impl<A: Attributes> Global for Html<marker::OptGroup, A> {}
impl<A: Attributes> Aria for Html<marker::OptGroup, A> {}

/// The `optgroup` element represents a group of [super::option] elements with a common label.
pub trait OptGroup: WithAttribute {
    /// Whether the form control is disabled.
    fn disabled(self) -> Self::Output<Disabled> {
        self.with_disabled(true)
    }

    /// Whether the form control is disabled.
    fn with_disabled(self, disabled: bool) -> Self::Output<Disabled> {
        self.with_attribute(Disabled(disabled))
    }

    /// User-visible label.
    fn label(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Label> {
        self.with_attribute(Label(value.into()))
    }
}
