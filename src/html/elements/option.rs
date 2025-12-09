use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::button::Disabled;
use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `option` element represents an option in a [super::select] element or as part of a list
/// of suggestions in a [super::datalist] element.
pub fn option() -> Element<marker::SelectOption> {
    Element::new("option")
}

pub mod marker {
    pub struct SelectOption;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, SelectOption> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> SelectOption<marker::SelectOption> for P where P: ElementProxy<marker::SelectOption> {}
impl<P> Common<marker::SelectOption> for P where P: ElementProxy<marker::SelectOption> {}
impl<P> Global<marker::SelectOption> for P where P: ElementProxy<marker::SelectOption> {}
impl<P> Aria<marker::SelectOption> for P where P: ElementProxy<marker::SelectOption> {}

/// The `option` element represents an option in a [super::select] element or as part of a list of
/// suggestions in a [super::datalist] element.
pub trait SelectOption<T>: WithAttribute {
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

    /// Whether the control is selected.
    fn selected(self) -> Self {
        self.with_selected(true)
    }

    /// Whether the control is selected.
    fn with_selected(self, selected: bool) -> Self {
        self.with_attribute(Selected(selected))
    }

    /// Value of the option
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Value(value.into()))
    }
}

/// User-visible label.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Label(pub Cow<'static, str>);

/// Whether the option is selected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Selected(pub bool);
