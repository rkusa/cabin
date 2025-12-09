use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::button::{Disabled, Form, Name};
use super::common::Common;
use super::global::Global;
use super::input::{AutoComplete, Dirname, MaxLength, MinLength, Placeholder, ReadOnly, Required};
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};
use crate::event::Event;
use crate::html::events::CustomEvent;

/// The `textarea` element represents a multiline plain text edit control for the element's raw
/// value. The contents of the control represent the control's default value.
pub fn textarea() -> Element<marker::Textarea> {
    Element::new("textarea")
}

pub mod marker {
    pub struct Textarea;

    impl<'v, S: Into<std::borrow::Cow<'v, str>>> crate::element::IntoChild<'v, Textarea> for S {
        fn into_child(self) -> impl crate::View + 'v {
            self.into()
        }
    }
}

impl<P> Textarea<marker::Textarea> for P where P: ElementProxy<marker::Textarea> {}
impl<P> Common<marker::Textarea> for P where P: ElementProxy<marker::Textarea> {}
impl<P> Global<marker::Textarea> for P where P: ElementProxy<marker::Textarea> {}
impl<P> Aria<marker::Textarea> for P where P: ElementProxy<marker::Textarea> {}

/// The `textarea` element represents a multiline plain text edit control for the element's raw
/// value. The contents of the control represent the control's default value.
pub trait Textarea<T>: WithAttribute {
    /// Hint for form autofill feature.
    fn autocomplete(self, autocomplete: AutoComplete) -> Self {
        self.with_attribute(autocomplete)
    }

    /// Maximum number of characters per line.
    fn cols(self, cols: u32) -> Self {
        self.with_attribute(Cols(cols))
    }

    /// Name of form control to use for sending the element's directionality in form submission.
    fn dirname(self, dirname: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Dirname(dirname.into()))
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

    /// Maximum length of value.
    fn max_length(self, max_length: i32) -> Self {
        self.with_attribute(MaxLength(max_length))
    }

    /// Minimum length of value.
    fn min_length(self, min_length: i32) -> Self {
        self.with_attribute(MinLength(min_length))
    }

    /// Name of the element to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }

    /// User-visible label to be placed within the form control.
    fn placeholder(self, placeholder: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Placeholder(placeholder.into()))
    }

    /// Whether to allow the value to be edited by the user.
    fn read_only(self) -> Self {
        self.with_read_only(true)
    }

    /// Whether to allow the value to be edited by the user.
    fn with_read_only(self, read_only: bool) -> Self {
        self.with_attribute(ReadOnly(read_only))
    }

    /// Whether the control is required for form submission.
    fn required(self) -> Self {
        self.with_required(true)
    }

    /// Whether the control is required for form submission.
    fn with_required(self, required: bool) -> Self {
        self.with_attribute(Required(required))
    }

    /// Number of lines to show.
    fn rows(self, rows: u32) -> Self {
        self.with_attribute(Rows(rows))
    }

    /// How the value of the form control is to be wrapped for form submission.
    fn wrap(self, wrap: Wrap) -> Self {
        self.with_attribute(wrap)
    }

    /// How the value of the form control is to be wrapped for form submission.
    fn wrap_hard(self) -> Self {
        self.with_attribute(Wrap::Hard)
    }

    fn on_input<E>(self, event: E) -> Self
    where
        E: ::serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("input", event))
    }

    fn on_change<E>(self, event: E) -> Self
    where
        E: ::serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("change", event))
    }
}

/// Maximum number of characters per line.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Cols(pub u32);

/// Number of lines to show.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Rows(pub u32);

/// Data type of an input element.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Wrap {
    /// The text is not to be wrapped when it is submitted.
    #[default]
    Soft,
    /// The text is to have newlines added by the user agent so that the text is wrapped when it is
    /// submitted.
    Hard,
}

impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Wrap::Soft => "soft",
            Wrap::Hard => "hard",
        })
    }
}
