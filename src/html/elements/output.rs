use std::borrow::Cow;

use super::aria::Aria;
use super::button::{Form, Name};
use super::common::Common;
use super::global::Global;
use super::label::For;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The output element represents the result of a calculation performed by the application, or
    /// the result of a user action.
    pub fn output(&self) -> Element<'_, marker::Output> {
        Element::new(self, "output")
    }
}

pub mod marker {
    pub struct Output;
}

impl<'v> Output for Element<'v, marker::Output> {}
impl<'v> Common for Element<'v, marker::Output> {}
impl<'v> Global for Element<'v, marker::Output> {}
impl<'v> Aria for Element<'v, marker::Output> {}

/// The output element represents the result of a calculation performed by the application, or the
/// result of a user action.
pub trait Output: WithAttribute {
    /// Specifies controls from which the output was calculated.
    fn r#for(self, r#for: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(For(r#for.into()))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Form(form.into()))
    }

    ///Name of the element to use in the `form.elements` API.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }
}
