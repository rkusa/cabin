use std::borrow::Cow;

use super::button::{Disabled, Form, Name};
use super::common::Common;
use super::global::Global;
use super::input::{AutoComplete, Multiple, OnChange, Required, Size};
use crate::View;
use crate::event::Event;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::events::CustomEvent;
use crate::html::{Aria, Html};

/// The `select` element represents a control for selecting amongst a set of [super::option]s.
pub fn select(content: impl View) -> Html<marker::Select, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("select", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! select {
        ($($x:tt)*) => {
            $crate::html::elements::select::select($crate::view![$($x)*])
        }
    }

    pub use select;
}

pub use macros::select;

pub mod marker {
    pub struct Select;
}

impl<A: Attributes> Select for Html<marker::Select, A> {}
impl<A: Attributes> Common for Html<marker::Select, A> {}
impl<A: Attributes> Global for Html<marker::Select, A> {}
impl<A: Attributes> Aria for Html<marker::Select, A> {}

/// The `select` element represents a control for selecting amongst a set of [super::option]s.
pub trait Select: WithAttribute {
    /// Hint for form autofill feature.
    fn autocomplete(self, autocomplete: AutoComplete) -> Self::Output<AutoComplete> {
        self.with_attribute(autocomplete)
    }

    /// Whether the form control is disabled.
    fn disabled(self) -> Self::Output<Disabled> {
        self.with_disabled(true)
    }

    /// Whether the form control is disabled.
    fn with_disabled(self, disabled: bool) -> Self::Output<Disabled> {
        self.with_attribute(Disabled(disabled))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self::Output<Form> {
        self.with_attribute(Form(form.into()))
    }

    /// Whether to allow multiple values.
    fn multiple(self) -> Self::Output<Multiple> {
        self.with_multiple(true)
    }

    /// Whether to allow multiple values.
    fn with_multiple(self, multiple: bool) -> Self::Output<Multiple> {
        self.with_attribute(Multiple(multiple))
    }

    /// Name of the element to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }

    /// Whether the control is required for form submission
    fn required(self) -> Self::Output<Required> {
        self.with_required(true)
    }

    /// Whether the control is required for form submission
    fn with_required(self, required: bool) -> Self::Output<Required> {
        self.with_attribute(Required(required))
    }

    /// Size of the control
    fn size(self, size: u32) -> Self::Output<Size> {
        self.with_attribute(Size(size))
    }

    fn on_change<E>(self, event: E) -> Self::Output<OnChange<E>>
    where
        E: ::serde::Serialize + Event + Send + 'static,
    {
        self.with_attribute(OnChange(CustomEvent::new("change", event)))
    }
}
