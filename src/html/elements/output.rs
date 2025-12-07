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
    pub fn output(&self) -> Element<marker::Output> {
        Element::new(self.acquire_renderer(), "output")
    }
}

pub mod marker {
    pub struct Output;
}

impl Output for Element<marker::Output> {}
impl Common for Element<marker::Output> {}
impl Global for Element<marker::Output> {}
impl Aria for Element<marker::Output> {}

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
