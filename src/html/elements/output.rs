use std::borrow::Cow;

use super::aria::Aria;
use super::button::{Form, Name};
use super::common::Common;
use super::global::Global;
use super::label::For;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The output element represents the result of a calculation performed by the application, or
/// the result of a user action.
pub fn output() -> Element<marker::Output> {
    Element::new("output")
}

pub mod marker {
    pub struct Output;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Output> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Output<marker::Output> for P where P: ElementProxy<marker::Output> {}
impl<P> Common<marker::Output> for P where P: ElementProxy<marker::Output> {}
impl<P> Global<marker::Output> for P where P: ElementProxy<marker::Output> {}
impl<P> Aria<marker::Output> for P where P: ElementProxy<marker::Output> {}

/// The output element represents the result of a calculation performed by the application, or the
/// result of a user action.
pub trait Output<T>: WithAttribute {
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
