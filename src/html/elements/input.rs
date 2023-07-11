mod auto_complete;

use std::borrow::Cow;
use std::fmt;

pub use auto_complete::AutoComplete;
use cabin_macros::Element;

use super::button::PopoverTargetAction;
use super::form::Method;
use crate::html::events::InputEvent;
use crate::html::SerializeEventFn;

// TODO: typed inputs? (number, date, ...)
/// An `a` element represents a typed data field, usually with a form control to allow the user to
/// edit data.
#[derive(Default, Element)]
#[element(void)]
pub struct Input<Ext = ()> {
    pub extension: Ext,

    #[element(event = InputEvent)]
    pub on_input: Option<Box<SerializeEventFn>>,

    /// Hint for expected file type in file upload controls.
    pub accept: Option<Cow<'static, str>>,

    /// Replacement text for use when images (type=image) are not available.
    pub alt: Option<Cow<'static, str>>,

    /// Hint for form autofill feature.
    pub autocomplete: Option<AutoComplete>,

    /// Whether the control is checked.
    pub checked: Option<bool>,

    /// Name of form control to use for sending the element's directionality in form submission.
    pub dirname: Option<Cow<'static, str>>,

    /// Whether the form control is disabled.
    pub disabled: Option<bool>,

    /// Associates the element with a form element.
    pub form: Option<Cow<'static, str>>,

    /// URL to use for form submission.
    #[element(attribute_name = "formaction")]
    pub form_action: Option<Cow<'static, str>>,

    /// Entry list encoding type to use for form submission.
    #[element(attribute_name = "formenctype")]
    pub form_enctype: Option<Cow<'static, str>>,

    /// Variant to use for form submission.
    #[element(attribute_name = "formmethod")]
    pub form_method: Option<Method>,

    /// Bypass form control validation for form submission.
    #[element(attribute_name = "formnovalidate")]
    pub form_novalidate: Option<Cow<'static, str>>,

    /// Navigable for form submission.
    #[element(attribute_name = "formtarget")]
    pub form_target: Option<Cow<'static, str>>,

    /// Vertical dimension.
    pub height: Option<u32>,

    /// List of autocomplete options.
    pub list: Option<Cow<'static, str>>,

    /// Maximum value
    pub max: Option<Cow<'static, str>>,

    /// Maximum length of value.
    #[element(attribute_name = "maxlength")]
    pub max_length: Option<i32>,

    /// Minimum value
    pub min: Option<Cow<'static, str>>,

    /// Minimum length of value
    #[element(attribute_name = "minlength")]
    pub min_length: Option<i32>,

    /// Whether to allow multiple values.
    pub multiple: Option<bool>,

    /// Name of the element to use for form submission.
    pub name: Option<Cow<'static, str>>,

    /// Pattern to be matched by the form control's value.
    pub pattern: Option<Cow<'static, str>>,

    /// User-visible label to be placed within the form control.
    pub placeholder: Option<Cow<'static, str>>,

    /// Targets a popover element to toggle, show, or hide.
    #[element(attribute_name = "popovertarget")]
    pub popover_target: Option<Cow<'static, str>>,

    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden
    #[element(attribute_name = "popovertargetaction")]
    pub popover_target_action: Option<PopoverTargetAction>,

    /// Whether to allow the value to be edited by the user
    #[element(attribute_name = "readonly")]
    pub read_only: Option<bool>,

    /// Whether the control is required for form submission
    pub required: Option<bool>,

    /// Size of the control
    pub size: Option<u32>,

    /// Address of the resource
    pub src: Option<Cow<'static, str>>,

    /// Granularity to be matched by the form control's value
    pub step: Option<Cow<'static, str>>,

    /// Type of form control.
    pub r#type: Option<Type>,

    /// Value of the form control
    pub value: Option<Cow<'static, str>>,

    /// Horizontal dimension.
    pub width: Option<u32>,
}

impl<Ext> InputElement<Ext> {
    pub fn type_hidden(mut self) -> Self {
        self.kind.r#type = Some(Type::Hidden);
        self
    }

    pub fn type_text(mut self) -> Self {
        self.kind.r#type = Some(Type::Text);
        self
    }

    pub fn type_search(mut self) -> Self {
        self.kind.r#type = Some(Type::Search);
        self
    }

    pub fn type_tel(mut self) -> Self {
        self.kind.r#type = Some(Type::Tel);
        self
    }

    pub fn type_url(mut self) -> Self {
        self.kind.r#type = Some(Type::Url);
        self
    }

    pub fn type_email(mut self) -> Self {
        self.kind.r#type = Some(Type::Email);
        self
    }

    pub fn type_password(mut self) -> Self {
        self.kind.r#type = Some(Type::Password);
        self
    }

    pub fn type_date(mut self) -> Self {
        self.kind.r#type = Some(Type::Date);
        self
    }

    pub fn type_month(mut self) -> Self {
        self.kind.r#type = Some(Type::Month);
        self
    }

    pub fn type_week(mut self) -> Self {
        self.kind.r#type = Some(Type::Week);
        self
    }

    pub fn type_time(mut self) -> Self {
        self.kind.r#type = Some(Type::Time);
        self
    }

    pub fn type_date_time_local(mut self) -> Self {
        self.kind.r#type = Some(Type::DateTimeLocal);
        self
    }

    pub fn type_number(mut self) -> Self {
        self.kind.r#type = Some(Type::Number);
        self
    }

    pub fn type_range(mut self) -> Self {
        self.kind.r#type = Some(Type::Range);
        self
    }

    pub fn type_color(mut self) -> Self {
        self.kind.r#type = Some(Type::Color);
        self
    }

    pub fn type_checkbox(mut self) -> Self {
        self.kind.r#type = Some(Type::Checkbox);
        self
    }

    pub fn type_radio(mut self) -> Self {
        self.kind.r#type = Some(Type::Radio);
        self
    }

    pub fn type_file(mut self) -> Self {
        self.kind.r#type = Some(Type::File);
        self
    }

    pub fn type_submit(mut self) -> Self {
        self.kind.r#type = Some(Type::Submit);
        self
    }

    pub fn type_image(mut self) -> Self {
        self.kind.r#type = Some(Type::Image);
        self
    }

    pub fn type_reset(mut self) -> Self {
        self.kind.r#type = Some(Type::Reset);
        self
    }

    pub fn type_button(mut self) -> Self {
        self.kind.r#type = Some(Type::Button);
        self
    }
}

/// Data type of an input element.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Type {
    Hidden,
    Text,
    Search,
    Tel,
    Url,
    Email,
    Password,
    Date,
    Month,
    Week,
    Time,
    DateTimeLocal,
    Number,
    Range,
    Color,
    Checkbox,
    Radio,
    File,
    Submit,
    Image,
    Reset,
    Button,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Type::Hidden => "hidden",
            Type::Text => "text",
            Type::Search => "search",
            Type::Tel => "tel",
            Type::Url => "url",
            Type::Email => "email",
            Type::Password => "password",
            Type::Date => "date",
            Type::Month => "month",
            Type::Week => "week",
            Type::Time => "time",
            Type::DateTimeLocal => "datetime-local",
            Type::Number => "number",
            Type::Range => "range",
            Type::Color => "color",
            Type::Checkbox => "checkbox",
            Type::Radio => "radio",
            Type::File => "file",
            Type::Submit => "submit",
            Type::Image => "image",
            Type::Reset => "reset",
            Type::Button => "button",
        })
    }
}
