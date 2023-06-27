use std::borrow::Cow;
use std::fmt;

use cabin_macros::Element;

use super::form::Method;

/// An `a` element that – if `href` is specified – creates a hyperlink to anything a URL can
/// address.
#[derive(Default, Element)]
pub struct Button {
    /// Whether the button is disabled.
    disabled: Option<bool>,

    /// Associates the button with a `form` element.
    form: Option<Cow<'static, str>>,

    /// The URL to use for form submission.
    #[element(attribute_name = "formaction")]
    form_action: Option<Cow<'static, str>>,

    /// Entry list encoding type to use for form submission.
    #[element(attribute_name = "formenctype")]
    form_enctype: Option<Cow<'static, str>>,

    /// Variant to use for form submission
    #[element(attribute_name = "formmethod")]
    form_method: Option<Method>,

    /// Bypass form control validation for form submission.
    #[element(attribute_name = "formnovalidate")]
    form_novalidate: Option<bool>,

    /// Navigable for form submission
    #[element(attribute_name = "formtarget")]
    form_target: Option<Cow<'static, str>>,

    /// Name of the button to use for form submission.
    name: Option<Cow<'static, str>>,

    /// ID of an element with a popover attribute.
    #[element(attribute_name = "popovertarget")]
    popover_target: Option<Cow<'static, str>>,

    /// The action to take on the targeted popover element.
    #[element(attribute_name = "popovertargetaction")]
    popover_target_action: Option<PopoverTargetAction>,

    /// Type of button.
    r#type: Option<Type>,

    /// Value to be used for form submission
    value: Option<Cow<'static, str>>,
}

/// The action to take on the targeted popover element.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            PopoverTargetAction::Toggle => f.write_str("toggle"),
            PopoverTargetAction::Show => f.write_str("show"),
            PopoverTargetAction::Hide => f.write_str("hide"),
        }
    }
}

/// The behavior of the button when activated.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            Type::Submit => f.write_str("submit"),
            Type::Reset => f.write_str("reset"),
            Type::Button => f.write_str("button"),
        }
    }
}