mod auto_complete;

use std::borrow::Cow;
use std::fmt;

pub use auto_complete::AutoComplete;
use cabin_macros::Attribute;

use super::aria::Aria;
use super::button::{
    Disabled, Form, FormAction, FormEnctype, FormMethod, FormNoValidate, FormTarget, Name,
    PopoverTarget, PopoverTargetAction,
};
use super::common::Common;
use super::global::Global;
use super::img::Alt;
use super::script::Src;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::event::Event;
use crate::html::events::CustomEvent;
use crate::void_element::VoidElement;

impl Context {
    pub fn input(&self) -> VoidElement<marker::Input> {
        VoidElement::new(self.acquire_renderer(), "input")
    }
}

pub mod marker {
    pub struct Input;
}

impl Input for VoidElement<marker::Input> {}
impl Common for VoidElement<marker::Input> {}
impl Global for VoidElement<marker::Input> {}
impl Aria for VoidElement<marker::Input> {}

// TODO: typed inputs? (number, date, ...)
/// TODO: doc comment
pub trait Input: WithAttribute {
    /// Hint for expected file type in file upload controls.
    fn accept(self, accept: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Accept(accept.into()))
    }

    /// Replacement text for use when images (type=image) are not available.
    fn alt(self, alt: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Alt(alt.into()))
    }

    /// Hint for form autofill feature.
    fn autocomplete(self, autocomplete: AutoComplete) -> Self {
        self.with_attribute(autocomplete)
    }

    /// Whether the control is checked.
    fn checked(self) -> Self {
        self.with_checked(true)
    }

    /// Whether the control is checked.
    fn with_checked(self, checked: bool) -> Self {
        self.with_attribute(Checked(checked))
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

    /// URL to use for form submission.
    fn form_action(self, form_action: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(FormAction(form_action.into()))
    }

    /// Entry list encoding type to use for form submission.
    fn form_enctype(self, form_enctype: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(FormEnctype(form_enctype.into()))
    }

    /// Variant to use for form submission.
    fn form_method(self, form_method: FormMethod) -> Self {
        self.with_attribute(form_method)
    }

    /// Bypass form control validation for form submission.
    fn form_novalidate(self) -> Self {
        self.with_form_novalidate(true)
    }

    /// Bypass form control validation for form submission.
    fn with_form_novalidate(self, form_novalidate: bool) -> Self {
        self.with_attribute(FormNoValidate(form_novalidate))
    }

    /// Navigable for form submission.
    fn form_target(self, form_target: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(FormTarget(form_target.into()))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self {
        self.with_attribute(Height(height))
    }

    /// List of autocomplete options.
    fn list(self, list: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(List(list.into()))
    }

    /// Maximum value
    fn max(self, max: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Max(max.into()))
    }

    /// Maximum length of value.
    fn max_length(self, max_length: i32) -> Self {
        self.with_attribute(MaxLength(max_length))
    }

    /// Minimum value
    fn min(self, min: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Min(min.into()))
    }

    /// Minimum length of value
    fn min_length(self, min_length: i32) -> Self {
        self.with_attribute(MinLength(min_length))
    }

    /// Whether to allow multiple values.
    fn multiple(self) -> Self {
        self.with_multiple(true)
    }

    /// Whether to allow multiple values.
    fn with_multiple(self, multiple: bool) -> Self {
        self.with_attribute(Multiple(multiple))
    }

    /// Name of the element to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }

    /// Pattern to be matched by the form control's value.
    fn pattern(self, pattern: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Pattern(pattern.into()))
    }

    /// User-visible label to be placed within the form control.
    fn placeholder(self, placeholder: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Placeholder(placeholder.into()))
    }

    /// Targets a popover element to toggle, show, or hide.
    fn popover_target(self, popover_target: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(PopoverTarget(popover_target.into()))
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden
    fn popover_target_action(self, popover_target_action: PopoverTargetAction) -> Self {
        self.with_attribute(popover_target_action)
    }

    /// Whether to allow the value to be edited by the user
    fn read_only(self) -> Self {
        self.with_read_only(true)
    }

    /// Whether to allow the value to be edited by the user
    fn with_read_only(self, read_only: bool) -> Self {
        self.with_attribute(ReadOnly(read_only))
    }

    /// Whether the control is required for form submission
    fn required(self) -> Self {
        self.with_required(true)
    }

    /// Whether the control is required for form submission
    fn with_required(self, required: bool) -> Self {
        self.with_attribute(Required(required))
    }

    /// Size of the control
    fn size(self, size: u32) -> Self {
        self.with_attribute(Size(size))
    }

    /// Address of the resource
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Src(src.into()))
    }

    /// Granularity to be matched by the form control's value
    fn step(self, step: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Step(step.into()))
    }

    /// Type of form control.
    fn r#type(self, r#type: Type) -> Self {
        self.with_attribute(r#type)
    }

    fn type_hidden(self) -> Self {
        self.r#type(Type::Hidden)
    }

    fn type_text(self) -> Self {
        self.r#type(Type::Text)
    }

    fn type_search(self) -> Self {
        self.r#type(Type::Search)
    }

    fn type_tel(self) -> Self {
        self.r#type(Type::Tel)
    }

    fn type_url(self) -> Self {
        self.r#type(Type::Url)
    }

    fn type_email(self) -> Self {
        self.r#type(Type::Email)
    }

    fn type_password(self) -> Self {
        self.r#type(Type::Password)
    }

    fn type_date(self) -> Self {
        self.r#type(Type::Date)
    }

    fn type_month(self) -> Self {
        self.r#type(Type::Month)
    }

    fn type_week(self) -> Self {
        self.r#type(Type::Week)
    }

    fn type_time(self) -> Self {
        self.r#type(Type::Time)
    }

    fn type_date_time_local(self) -> Self {
        self.r#type(Type::DateTimeLocal)
    }

    fn type_number(self) -> Self {
        self.r#type(Type::Number)
    }

    fn type_range(self) -> Self {
        self.r#type(Type::Range)
    }

    fn type_color(self) -> Self {
        self.r#type(Type::Color)
    }

    fn type_checkbox(self) -> Self {
        self.r#type(Type::Checkbox)
    }

    fn type_radio(self) -> Self {
        self.r#type(Type::Radio)
    }

    fn type_file(self) -> Self {
        self.r#type(Type::File)
    }

    fn type_submit(self) -> Self {
        self.r#type(Type::Submit)
    }

    fn type_image(self) -> Self {
        self.r#type(Type::Image)
    }

    fn type_reset(self) -> Self {
        self.r#type(Type::Reset)
    }

    fn type_button(self) -> Self {
        self.r#type(Type::Button)
    }

    /// Value of the form control
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Value(value.into()))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self {
        self.with_attribute(Width(width))
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

/// Hint for expected file type in file upload controls.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Accept(pub Cow<'static, str>);

/// Whether the control is checked.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Checked(pub bool);

/// Name of form control to use for sending the element's directionality in form submission.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Dirname(pub Cow<'static, str>);

/// Vertical dimension.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Height(pub u32);

/// List of autocomplete options.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct List(pub Cow<'static, str>);

/// Maximum value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Max(pub Cow<'static, str>);

/// Maximum length of value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct MaxLength(pub i32);

/// Minimum value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Min(pub Cow<'static, str>);

/// Minimum length of value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct MinLength(pub i32);

/// Whether to allow multiple values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Multiple(pub bool);

/// Pattern to be matched by the form control's value.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Pattern(pub Cow<'static, str>);

/// User-visible label to be placed within the form control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Placeholder(pub Cow<'static, str>);

/// Whether to allow the value to be edited by the user
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ReadOnly(pub bool);

/// Whether the control is required for form submission
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Required(pub bool);

/// Size of the control
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Size(pub u32);

/// Granularity to be matched by the form control's value
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Step(pub Cow<'static, str>);

/// Value of the form control
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Value(pub Cow<'static, str>);

/// Horizontal dimension.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Width(pub u32);

/// Data type of an input element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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
            Self::Hidden => "hidden",
            Self::Text => "text",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Email => "email",
            Self::Password => "password",
            Self::Date => "date",
            Self::Month => "month",
            Self::Week => "week",
            Self::Time => "time",
            Self::DateTimeLocal => "datetime-local",
            Self::Number => "number",
            Self::Range => "range",
            Self::Color => "color",
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
            Self::File => "file",
            Self::Submit => "submit",
            Self::Image => "image",
            Self::Reset => "reset",
            Self::Button => "button",
        })
    }
}
