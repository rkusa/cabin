use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

pub fn option(content: impl View) -> Html<marker::SelectOption, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("option", (), content)
}

pub mod marker {
    pub struct SelectOption;
}

impl<A: Attributes, V: 'static> SelectOption for Html<marker::SelectOption, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::SelectOption, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::SelectOption, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::SelectOption, A, V> {}

// TODO:
pub trait SelectOption: WithAttribute {
    /// Value of the option
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }

    /// Whether the control is checked.
    fn selected(self) -> Self::Output<Selected> {
        self.with_selected(true)
    }

    /// Whether the control is selected.
    fn with_selected(self, selected: bool) -> Self::Output<Selected> {
        self.with_attribute(Selected(selected))
    }
}

/// Whether the option is selected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Selected(pub bool);
