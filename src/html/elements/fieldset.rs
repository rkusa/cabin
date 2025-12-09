use std::borrow::Cow;

use super::aria::Aria;
use super::button::{Disabled, Form, Name};
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `fieldset` element represents a set of form controls (or other content) grouped
/// together, optionally with a caption. The caption is given by the first [super::legend]
/// element that is a child of the [super::fieldset] element, if any. The remainder of the
/// descendants form the group.
pub fn fieldset() -> Element<marker::Fieldset> {
    Element::new("fieldset")
}

pub mod marker {
    pub struct Fieldset;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Fieldset> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Fieldset<marker::Fieldset> for P where P: ElementProxy<marker::Fieldset> {}
impl<P> Common<marker::Fieldset> for P where P: ElementProxy<marker::Fieldset> {}
impl<P> Global<marker::Fieldset> for P where P: ElementProxy<marker::Fieldset> {}
impl<P> Aria<marker::Fieldset> for P where P: ElementProxy<marker::Fieldset> {}

/// The `fieldset` element represents a set of form controls (or other content) grouped together,
/// optionally with a caption. The caption is given by the first [super::legend] element that is a
/// child of the [super::fieldset] element, if any. The remainder of the descendants form the group.
pub trait Fieldset<T>: WithAttribute {
    /// Whether the descendant form controls, except any inside [super::legend], are disabled.
    fn disabled(self) -> Self {
        self.with_disabled(true)
    }

    /// Whether the descendant form controls, except any inside [super::legend], are disabled.
    fn with_disabled(self, disabled: bool) -> Self {
        self.with_attribute(Disabled(disabled))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Form(form.into()))
    }

    /// Name of the element to use in the `form.elements` API.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }
}
