use std::borrow::Cow;

use cabin_macros::Attribute;

use super::button::Disabled;
use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `option` element represents an option in a [super::select] element or as part of a list of
/// suggestions in a [super::datalist] element.
pub fn option(content: impl View) -> Html<marker::SelectOption, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("option", (), content)
}

pub mod marker {
    pub struct SelectOption;
}

impl<A: Attributes> SelectOption for Html<marker::SelectOption, A> {}
impl<A: Attributes> Common for Html<marker::SelectOption, A> {}
impl<A: Attributes> Global for Html<marker::SelectOption, A> {}
impl<A: Attributes> Aria for Html<marker::SelectOption, A> {}

/// The `option` element represents an option in a [super::select] element or as part of a list of
/// suggestions in a [super::datalist] element.
pub trait SelectOption: WithAttribute {
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

    /// Whether the control is selected.
    fn selected(self) -> Self::Output<Selected> {
        self.with_selected(true)
    }

    /// Whether the control is selected.
    fn with_selected(self, selected: bool) -> Self::Output<Selected> {
        self.with_attribute(Selected(selected))
    }

    /// Value of the option
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }
}

/// User-visible label.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Label(pub Cow<'static, str>);

/// Whether the option is selected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Selected(pub bool);
