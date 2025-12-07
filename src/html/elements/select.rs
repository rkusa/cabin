use std::borrow::Cow;

use super::aria::Aria;
use super::button::{Disabled, Form, Name};
use super::common::Common;
use super::global::Global;
use super::input::{AutoComplete, Multiple, Required, Size};
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;
use crate::event::Event;
use crate::html::events::CustomEvent;

impl Context {
    /// The `select` element represents a control for selecting amongst a set of [super::option]s.
    pub fn select(&self) -> Element<marker::Select> {
        Element::new(self.acquire_renderer(), "select")
    }
}

pub mod marker {
    pub struct Select;
}

impl Select for Element<marker::Select> {}
impl Common for Element<marker::Select> {}
impl Global for Element<marker::Select> {}
impl Aria for Element<marker::Select> {}

/// The `select` element represents a control for selecting amongst a set of [super::option]s.
pub trait Select: WithAttribute {
    /// Hint for form autofill feature.
    fn autocomplete(self, autocomplete: AutoComplete) -> Self {
        self.with_attribute(autocomplete)
    }

    /// Whether the form control is disabled.
    fn disabled(self) -> Self {
        self.with_disabled(true)
    }

    /// Whether the form control is disabled.
    fn with_disabled(self, disabled: bool) -> Self {
        self.with_attribute(Disabled(disabled))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Form(form.into()))
    }

    /// Whether to allow multiple values.
    fn multiple(self) -> Self {
        self.with_multiple(true)
    }

    /// Whether to allow multiple values.
    fn with_multiple(self, multiple: bool) -> Self {
        self.with_attribute(Multiple(multiple))
    }

    /// Name of the element to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }

    /// Whether the control is required for form submission
    fn required(self) -> Self {
        self.with_required(true)
    }

    /// Whether the control is required for form submission
    fn with_required(self, required: bool) -> Self {
        self.with_attribute(Required(required))
    }

    /// Size of the control
    fn size(self, size: u32) -> Self {
        self.with_attribute(Size(size))
    }

    fn on_change<E>(self, event: E) -> Self
    where
        E: ::serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("change", event))
    }
}
