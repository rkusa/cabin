use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::event::Event;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::form::OnSubmit;
use crate::html::{Aria, Html};

/// The `button` element represents a button labeled by its contents.
#[crate::view_macro(crate::html::elements::button)]
pub fn button(content: impl View) -> Html<marker::Button, ()> {
    Html::new("button", (), content)
}

pub mod marker {
    pub struct Button;
}

impl<A: Attributes> Button for Html<marker::Button, A> {}
impl<A: Attributes> Common for Html<marker::Button, A> {}
impl<A: Attributes> Global for Html<marker::Button, A> {}
impl<A: Attributes> Aria for Html<marker::Button, A> {}

/// The `button` element represents a button labeled by its contents.
pub trait Button: WithAttribute {
    /// Whether the button is disabled.
    fn disabled(self) -> Self::Output<Disabled> {
        self.with_disabled(true)
    }

    /// Whether the button is disabled.
    fn with_disabled(self, disabled: bool) -> Self::Output<Disabled> {
        self.with_attribute(Disabled(disabled))
    }

    /// Associates the button with a `form` element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self::Output<Form> {
        self.with_attribute(Form(form.into()))
    }

    /// The URL to use for form submission.
    fn form_action(self, form_action: impl Into<Cow<'static, str>>) -> Self::Output<FormAction> {
        self.with_attribute(FormAction(form_action.into()))
    }

    /// Entry list encoding type to use for form submission.
    fn form_enctype(self, form_enctype: impl Into<Cow<'static, str>>) -> Self::Output<FormEnctype> {
        self.with_attribute(FormEnctype(form_enctype.into()))
    }

    /// Variant to use for form submission
    fn form_method(self, form_method: FormMethod) -> Self::Output<FormMethod> {
        self.with_attribute(form_method)
    }

    /// Bypass form control validation for form submission.
    fn form_novalidate(self) -> Self::Output<FormNoValidate> {
        self.with_form_novalidate(true)
    }

    /// Bypass form control validation for form submission.
    fn with_form_novalidate(self, form_novalidate: bool) -> Self::Output<FormNoValidate> {
        self.with_attribute(FormNoValidate(form_novalidate))
    }

    /// Navigable for form submission
    fn form_target(self, form_target: impl Into<Cow<'static, str>>) -> Self::Output<FormTarget> {
        self.with_attribute(FormTarget(form_target.into()))
    }

    /// Name of the button to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }

    /// ID of an element with a popover attribute.
    fn popover_target(
        self,
        popover_target: impl Into<Cow<'static, str>>,
    ) -> Self::Output<PopoverTarget> {
        self.with_attribute(PopoverTarget(popover_target.into()))
    }

    /// The action to take on the targeted popover element.
    fn popover_target_action(
        self,
        popover_target_action: PopoverTargetAction,
    ) -> Self::Output<PopoverTargetAction> {
        self.with_attribute(popover_target_action)
    }

    fn type_submit(self) -> Self::Output<Type> {
        self.r#type(Type::Submit)
    }

    fn type_reset(self) -> Self::Output<Type> {
        self.r#type(Type::Reset)
    }

    fn type_button(self) -> Self::Output<Type> {
        self.r#type(Type::Button)
    }

    /// Type of button.
    fn r#type(self, r#type: Type) -> Self::Output<Type> {
        self.with_attribute(r#type)
    }

    /// Value to be used for form submission
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }

    /// If a form is submitted with this button, the form elements are serialized into the `E`
    /// event regardless of what is set for the forms `on_submit`.
    fn on_submit<E>(self) -> Self::Output<OnSubmit>
    where
        E: Event + 'static,
    {
        self.with_attribute(OnSubmit(E::ID))
    }
}

/// Whether the element is disabled.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Disabled(pub bool);

/// Associates the button with a `form` element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Form(pub Cow<'static, str>);

/// The URL to use for form submission.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct FormAction(pub Cow<'static, str>);

/// Entry list encoding type to use for form submission.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct FormEnctype(pub Cow<'static, str>);

/// Bypass form control validation for form submission.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct FormNoValidate(pub bool);

/// Navigable for form submission
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct FormTarget(pub Cow<'static, str>);

/// Name of the button to use for form submission.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);

/// ID of an element with a popover attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct PopoverTarget(pub Cow<'static, str>);

/// Value to be used for form submission
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Value(pub Cow<'static, str>);

/// Variant used for form submission.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum FormMethod {
    /// Submit as GET request.
    #[default]
    Get,

    /// Submit as POST request.
    Post,

    /// Close dialog form is content of.
    Dialog,
}

impl fmt::Display for FormMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => f.write_str("get"),
            Self::Post => f.write_str("post"),
            Self::Dialog => f.write_str("dialog"),
        }
    }
}

/// The action to take on the targeted popover element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum PopoverTargetAction {
    /// Shows or hides the targeted popover element.
    Toggle,

    /// Shows the targeted popover element.
    Show,

    /// Hides the targeted popover element.
    Hide,
}

impl fmt::Display for PopoverTargetAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toggle => f.write_str("toggle"),
            Self::Show => f.write_str("show"),
            Self::Hide => f.write_str("hide"),
        }
    }
}

/// The behavior of the button when activated.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Type {
    /// Submits the form.
    Submit,

    /// Resets the form.
    Reset,

    /// Does nothing.
    Button,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submit => f.write_str("submit"),
            Self::Reset => f.write_str("reset"),
            Self::Button => f.write_str("button"),
        }
    }
}
