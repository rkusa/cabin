mod auto_complete;

use std::borrow::Cow;
use std::fmt;

pub use auto_complete::AutoComplete;
use cabin_macros::{element, Attribute};

use super::button::{
    Disabled, Form, FormAction, FormEnctype, FormMethod, FormNovalidate, FormTarget, Name,
    PopoverTarget, PopoverTargetAction,
};
use super::common::Common;
use super::global::Global;
use super::SerializeEventFn;
use crate::error::InternalError;
use crate::html::attributes::Attributes;
use crate::html::events::InputEvent;
use crate::html::Aria;

// TODO: typed inputs? (number, date, ...)
/// TODO: doc comment
#[element(void)]
pub trait Input: Common + Global + Aria {
    /// Hint for expected file type in file upload controls.
    fn accept(self, accept: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Accept(accept.into()))
    }

    /// Replacement text for use when images (type=image) are not available.
    fn alt(self, alt: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Alt(alt.into()))
    }

    /// Hint for form autofill feature.
    fn autocomplete(self, autocomplete: AutoComplete) -> impl Input {
        self.with(autocomplete)
    }

    /// Whether the control is checked.
    fn checked(self) -> impl Input {
        self.with_checked(true)
    }

    /// Whether the control is checked.
    fn with_checked(self, checked: bool) -> impl Input {
        self.with(Checked(checked))
    }

    /// Name of form control to use for sending the element's directionality in form submission.
    fn dirname(self, dirname: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Dirname(dirname.into()))
    }

    /// Whether the form control is disabled.
    fn disabled(self) -> impl Input {
        self.with_disabled(true)
    }

    /// Whether the form control is disabled.
    fn with_disabled(self, disabled: bool) -> impl Input {
        self.with(Disabled(disabled))
    }

    /// Associates the element with a form element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Form(form.into()))
    }

    /// URL to use for form submission.
    fn form_action(self, form_action: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(FormAction(form_action.into()))
    }

    /// Entry list encoding type to use for form submission.
    fn form_enctype(self, form_enctype: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(FormEnctype(form_enctype.into()))
    }

    /// Variant to use for form submission.
    fn form_method(self, form_method: FormMethod) -> impl Input {
        self.with(form_method)
    }

    /// Bypass form control validation for form submission.
    fn form_novalidate(self) -> impl Input {
        self.with_form_novalidate(true)
    }

    /// Bypass form control validation for form submission.
    fn with_form_novalidate(self, form_novalidate: bool) -> impl Input {
        self.with(FormNovalidate(form_novalidate))
    }

    /// Navigable for form submission.
    fn form_target(self, form_target: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(FormTarget(form_target.into()))
    }

    /// Vertical dimension.
    fn height(self, height: impl Into<u32>) -> impl Input {
        self.with(Height(height.into()))
    }

    /// List of autocomplete options.
    fn list(self, list: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(List(list.into()))
    }

    /// Maximum value
    fn max(self, max: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Max(max.into()))
    }

    /// Maximum length of value.
    fn max_length(self, max_length: impl Into<i32>) -> impl Input {
        self.with(MaxLength(max_length.into()))
    }

    /// Minimum value
    fn min(self, min: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Min(min.into()))
    }

    /// Minimum length of value
    fn min_length(self, min_length: impl Into<i32>) -> impl Input {
        self.with(MinLength(min_length.into()))
    }

    /// Whether to allow multiple values.
    fn multiple(self) -> impl Input {
        self.with_multiple(true)
    }

    /// Whether to allow multiple values.
    fn with_multiple(self, multiple: bool) -> impl Input {
        self.with(Multiple(multiple))
    }

    /// Name of the element to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Name(name.into()))
    }

    /// Pattern to be matched by the form control's value.
    fn pattern(self, pattern: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Pattern(pattern.into()))
    }

    /// User-visible label to be placed within the form control.
    fn placeholder(self, placeholder: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Placeholder(placeholder.into()))
    }

    /// Targets a popover element to toggle, show, or hide.
    fn popover_target(self, popover_target: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(PopoverTarget(popover_target.into()))
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden
    fn popover_target_action(self, popover_target_action: PopoverTargetAction) -> impl Input {
        self.with(popover_target_action)
    }

    /// Whether to allow the value to be edited by the user
    fn read_only(self) -> impl Input {
        self.with_read_only(true)
    }

    /// Whether to allow the value to be edited by the user
    fn with_read_only(self, read_only: bool) -> impl Input {
        self.with(ReadOnly(read_only))
    }

    /// Whether the control is required for form submission
    fn required(self) -> impl Input {
        self.with_required(true)
    }

    /// Whether the control is required for form submission
    fn with_required(self, required: bool) -> impl Input {
        self.with(Required(required))
    }

    /// Size of the control
    fn size(self, size: impl Into<u32>) -> impl Input {
        self.with(Size(size.into()))
    }

    /// Address of the resource
    fn src(self, src: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Src(src.into()))
    }

    /// Granularity to be matched by the form control's value
    fn step(self, step: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Step(step.into()))
    }

    /// Type of form control.
    fn r#type(self, r#type: Type) -> impl Input {
        self.with(r#type)
    }

    fn type_hidden(self) -> impl Input {
        self.r#type(Type::Hidden)
    }

    fn type_text(self) -> impl Input {
        self.r#type(Type::Text)
    }

    fn type_search(self) -> impl Input {
        self.r#type(Type::Search)
    }

    fn type_tel(self) -> impl Input {
        self.r#type(Type::Tel)
    }

    fn type_url(self) -> impl Input {
        self.r#type(Type::Url)
    }

    fn type_email(self) -> impl Input {
        self.r#type(Type::Email)
    }

    fn type_password(self) -> impl Input {
        self.r#type(Type::Password)
    }

    fn type_date(self) -> impl Input {
        self.r#type(Type::Date)
    }

    fn type_month(self) -> impl Input {
        self.r#type(Type::Month)
    }

    fn type_week(self) -> impl Input {
        self.r#type(Type::Week)
    }

    fn type_time(self) -> impl Input {
        self.r#type(Type::Time)
    }

    fn type_date_time_local(self) -> impl Input {
        self.r#type(Type::DateTimeLocal)
    }

    fn type_number(self) -> impl Input {
        self.r#type(Type::Number)
    }

    fn type_range(self) -> impl Input {
        self.r#type(Type::Range)
    }

    fn type_color(self) -> impl Input {
        self.r#type(Type::Color)
    }

    fn type_checkbox(self) -> impl Input {
        self.r#type(Type::Checkbox)
    }

    fn type_radio(self) -> impl Input {
        self.r#type(Type::Radio)
    }

    fn type_file(self) -> impl Input {
        self.r#type(Type::File)
    }

    fn type_submit(self) -> impl Input {
        self.r#type(Type::Submit)
    }

    fn type_image(self) -> impl Input {
        self.r#type(Type::Image)
    }

    fn type_reset(self) -> impl Input {
        self.r#type(Type::Reset)
    }

    fn type_button(self) -> impl Input {
        self.r#type(Type::Button)
    }

    /// Value of the form control
    fn value(self, value: impl Into<Cow<'static, str>>) -> impl Input {
        self.with(Value(value.into()))
    }

    /// Horizontal dimension.
    fn width(self, width: impl Into<u32>) -> impl Input {
        self.with(Width(width.into()))
    }

    fn on_input<E>(self, event: impl FnOnce(InputEvent) -> E) -> impl Input
    where
        E: ::serde::Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.with(OnInput(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_input event",
                    err,
                })
                .map(|json| (hash, json))
        })))
    }

    fn on_change<E>(self, event: impl FnOnce(InputEvent) -> E) -> impl Input
    where
        E: ::serde::Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.with(OnChange(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_change event",
                    err,
                })
                .map(|json| (hash, json))
        })))
    }
}

/// Hint for expected file type in file upload controls.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Accept(pub Cow<'static, str>);

/// Replacement text for use when images (type=image) are not available.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Alt(pub Cow<'static, str>);

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

/// Address of the resource
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Src(pub Cow<'static, str>);

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

pub struct OnInput(pub Box<SerializeEventFn>);

impl Attributes for OnInput {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        // TODO: directly write into el?
        let (id, payload) = &(self.0)()?;
        r.attribute("cabin-input", id)
            .map_err(crate::error::InternalError::from)?;
        r.attribute("cabin-input-payload", payload)
            .map_err(crate::error::InternalError::from)?;

        Ok(())
    }
}

pub struct OnChange(pub Box<SerializeEventFn>);

impl Attributes for OnChange {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        // TODO: directly write into el?
        let (id, payload) = &(self.0)()?;
        r.attribute("cabin-change", id)
            .map_err(crate::error::InternalError::from)?;
        r.attribute("cabin-change-payload", payload)
            .map_err(crate::error::InternalError::from)?;

        Ok(())
    }
}
