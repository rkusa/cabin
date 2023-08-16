use std::borrow::Cow;

use super::button::{Form, Name};
use super::common::Common;
use super::global::Global;
use super::label::For;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

/// The output element represents the result of a calculation performed by the application, or the
/// result of a user action.
pub fn output(content: impl View) -> Html<marker::Output, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("output", (), content)
}

pub mod marker {
    pub struct Output;
}

impl<A: Attributes, V: 'static> Output for Html<marker::Output, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Output, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Output, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Output, A, V> {}

/// The output element represents the result of a calculation performed by the application, or the
/// result of a user action.
pub trait Output: WithAttribute {
    /// Specifies controls from which the output was calculated.
    fn r#for(self, r#for: impl Into<Cow<'static, str>>) -> Self::Output<For> {
        self.with_attribute(For(r#for.into()))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self::Output<Form> {
        self.with_attribute(Form(form.into()))
    }

    ///Name of the element to use in the `form.elements` API.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }
}
