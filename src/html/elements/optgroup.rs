use std::borrow::Cow;

use super::aria::Aria;
use super::button::Disabled;
use super::common::Common;
use super::global::Global;
use super::option::Label;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `optgroup` element represents a group of [super::option] elements with a common label.
pub fn optgroup() -> Element<marker::OptGroup> {
    Element::new("optgroup")
}

pub mod marker {
    pub struct OptGroup;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, OptGroup> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> OptGroup<marker::OptGroup> for P where P: ElementProxy<marker::OptGroup> {}
impl<P> Common<marker::OptGroup> for P where P: ElementProxy<marker::OptGroup> {}
impl<P> Global<marker::OptGroup> for P where P: ElementProxy<marker::OptGroup> {}
impl<P> Aria<marker::OptGroup> for P where P: ElementProxy<marker::OptGroup> {}

/// The `optgroup` element represents a group of [super::option] elements with a common label.
pub trait OptGroup<T>: WithAttribute {
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
