use std::borrow::Cow;

use super::button::{Disabled, Form, Name};
use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `fieldset` element represents a set of form controls (or other content) grouped together,
/// optionally with a caption. The caption is given by the first [super::legend] element that is a
/// child of the [super::fieldset] element, if any. The remainder of the descendants form the group.
pub fn fieldset(content: impl View) -> Html<marker::Fieldset, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("fieldset", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! fieldset {
        ($($x:tt)*) => {
            $crate::html::elements::fieldset::fieldset($crate::view![$($x)*])
        }
    }

    pub use fieldset;
}

pub use macros::fieldset;

pub mod marker {
    pub struct Fieldset;
}

impl<A: Attributes> Fieldset for Html<marker::Fieldset, A> {}
impl<A: Attributes> Common for Html<marker::Fieldset, A> {}
impl<A: Attributes> Global for Html<marker::Fieldset, A> {}
impl<A: Attributes> Aria for Html<marker::Fieldset, A> {}

/// The `fieldset` element represents a set of form controls (or other content) grouped together,
/// optionally with a caption. The caption is given by the first [super::legend] element that is a
/// child of the [super::fieldset] element, if any. The remainder of the descendants form the group.
pub trait Fieldset: WithAttribute {
    /// Whether the descendant form controls, except any inside [super::legend], are disabled.
    fn disabled(self) -> Self::Output<Disabled> {
        self.with_disabled(true)
    }

    /// Whether the descendant form controls, except any inside [super::legend], are disabled.
    fn with_disabled(self, disabled: bool) -> Self::Output<Disabled> {
        self.with_attribute(Disabled(disabled))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self::Output<Form> {
        self.with_attribute(Form(form.into()))
    }

    /// Name of the element to use in the `form.elements` API.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }
}
