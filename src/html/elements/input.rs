mod auto_complete;

use std::borrow::Cow;
use std::fmt;

pub use auto_complete::AutoComplete;
use cabin_macros::Attribute;

use super::button::{
    Disabled, Form, FormAction, FormEnctype, FormMethod, FormNoValidate, FormTarget, Name,
    PopoverTarget, PopoverTargetAction,
};
use super::common::Common;
use super::global::Global;
use super::img::Alt;
use super::script::Src;
use super::SerializeEventFn;
use crate::error::InternalError;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::events::InputEvent;
use crate::html::{Aria, Html};

pub fn input() -> Html<marker::Input, (), ()> {
    Html::new("input", (), ()).into_void_element()
}

pub mod marker {
    pub struct Input;
}

impl<A: Attributes, V: 'static> Input for Html<marker::Input, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Input, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Input, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Input, A, V> {}

// TODO: typed inputs? (number, date, ...)
/// TODO: doc comment
pub trait Input: WithAttribute {
    /// Hint for expected file type in file upload controls.
    fn accept(self, accept: impl Into<Cow<'static, str>>) -> Self::Output<Accept> {
        self.with_attribute(Accept(accept.into()))
    }

    /// Replacement text for use when images (type=image) are not available.
    fn alt(self, alt: impl Into<Cow<'static, str>>) -> Self::Output<Alt> {
        self.with_attribute(Alt(alt.into()))
    }

    /// Hint for form autofill feature.
    fn autocomplete(self, autocomplete: AutoComplete) -> Self::Output<AutoComplete> {
        self.with_attribute(autocomplete)
    }

    /// Whether the control is checked.
    fn checked(self) -> Self::Output<Checked> {
        self.with_checked(true)
    }

    /// Whether the control is checked.
    fn with_checked(self, checked: bool) -> Self::Output<Checked> {
        self.with_attribute(Checked(checked))
    }

    /// Name of form control to use for sending the element's directionality in form submission.
    fn dirname(self, dirname: impl Into<Cow<'static, str>>) -> Self::Output<Dirname> {
        self.with_attribute(Dirname(dirname.into()))
    }

    /// Whether the form control is disabled.
    fn disabled(self) -> Self::Output<Disabled> {
        self.with_disabled(true)
    }

    /// Whether the form control is disabled.
    fn with_disabled(self, disabled: bool) -> Self::Output<Disabled> {
        self.with_attribute(Disabled(disabled))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self::Output<Form> {
        self.with_attribute(Form(form.into()))
    }

    /// URL to use for form submission.
    fn form_action(self, form_action: impl Into<Cow<'static, str>>) -> Self::Output<FormAction> {
        self.with_attribute(FormAction(form_action.into()))
    }

    /// Entry list encoding type to use for form submission.
    fn form_enctype(self, form_enctype: impl Into<Cow<'static, str>>) -> Self::Output<FormEnctype> {
        self.with_attribute(FormEnctype(form_enctype.into()))
    }

    /// Variant to use for form submission.
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

    /// Navigable for form submission.
    fn form_target(self, form_target: impl Into<Cow<'static, str>>) -> Self::Output<FormTarget> {
        self.with_attribute(FormTarget(form_target.into()))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self::Output<Height> {
        self.with_attribute(Height(height))
    }

    /// List of autocomplete options.
    fn list(self, list: impl Into<Cow<'static, str>>) -> Self::Output<List> {
        self.with_attribute(List(list.into()))
    }

    /// Maximum value
    fn max(self, max: impl Into<Cow<'static, str>>) -> Self::Output<Max> {
        self.with_attribute(Max(max.into()))
    }

    /// Maximum length of value.
    fn max_length(self, max_length: i32) -> Self::Output<MaxLength> {
        self.with_attribute(MaxLength(max_length))
    }

    /// Minimum value
    fn min(self, min: impl Into<Cow<'static, str>>) -> Self::Output<Min> {
        self.with_attribute(Min(min.into()))
    }

    /// Minimum length of value
    fn min_length(self, min_length: i32) -> Self::Output<MinLength> {
        self.with_attribute(MinLength(min_length))
    }

    /// Whether to allow multiple values.
    fn multiple(self) -> Self::Output<Multiple> {
        self.with_multiple(true)
    }

    /// Whether to allow multiple values.
    fn with_multiple(self, multiple: bool) -> Self::Output<Multiple> {
        self.with_attribute(Multiple(multiple))
    }

    /// Name of the element to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }

    /// Pattern to be matched by the form control's value.
    fn pattern(self, pattern: impl Into<Cow<'static, str>>) -> Self::Output<Pattern> {
        self.with_attribute(Pattern(pattern.into()))
    }

    /// User-visible label to be placed within the form control.
    fn placeholder(self, placeholder: impl Into<Cow<'static, str>>) -> Self::Output<Placeholder> {
        self.with_attribute(Placeholder(placeholder.into()))
    }

    /// Targets a popover element to toggle, show, or hide.
    fn popover_target(
        self,
        popover_target: impl Into<Cow<'static, str>>,
    ) -> Self::Output<PopoverTarget> {
        self.with_attribute(PopoverTarget(popover_target.into()))
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden
    fn popover_target_action(
        self,
        popover_target_action: PopoverTargetAction,
    ) -> Self::Output<PopoverTargetAction> {
        self.with_attribute(popover_target_action)
    }

    /// Whether to allow the value to be edited by the user
    fn read_only(self) -> Self::Output<ReadOnly> {
        self.with_read_only(true)
    }

    /// Whether to allow the value to be edited by the user
    fn with_read_only(self, read_only: bool) -> Self::Output<ReadOnly> {
        self.with_attribute(ReadOnly(read_only))
    }

    /// Whether the control is required for form submission
    fn required(self) -> Self::Output<Required> {
        self.with_required(true)
    }

    /// Whether the control is required for form submission
    fn with_required(self, required: bool) -> Self::Output<Required> {
        self.with_attribute(Required(required))
    }

    /// Size of the control
    fn size(self, size: u32) -> Self::Output<Size> {
        self.with_attribute(Size(size))
    }

    /// Address of the resource
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Src> {
        self.with_attribute(Src(src.into()))
    }

    /// Granularity to be matched by the form control's value
    fn step(self, step: impl Into<Cow<'static, str>>) -> Self::Output<Step> {
        self.with_attribute(Step(step.into()))
    }

    /// Type of form control.
    fn r#type(self, r#type: Type) -> Self::Output<Type> {
        self.with_attribute(r#type)
    }

    fn type_hidden(self) -> Self::Output<Type> {
        self.r#type(Type::Hidden)
    }

    fn type_text(self) -> Self::Output<Type> {
        self.r#type(Type::Text)
    }

    fn type_search(self) -> Self::Output<Type> {
        self.r#type(Type::Search)
    }

    fn type_tel(self) -> Self::Output<Type> {
        self.r#type(Type::Tel)
    }

    fn type_url(self) -> Self::Output<Type> {
        self.r#type(Type::Url)
    }

    fn type_email(self) -> Self::Output<Type> {
        self.r#type(Type::Email)
    }

    fn type_password(self) -> Self::Output<Type> {
        self.r#type(Type::Password)
    }

    fn type_date(self) -> Self::Output<Type> {
        self.r#type(Type::Date)
    }

    fn type_month(self) -> Self::Output<Type> {
        self.r#type(Type::Month)
    }

    fn type_week(self) -> Self::Output<Type> {
        self.r#type(Type::Week)
    }

    fn type_time(self) -> Self::Output<Type> {
        self.r#type(Type::Time)
    }

    fn type_date_time_local(self) -> Self::Output<Type> {
        self.r#type(Type::DateTimeLocal)
    }

    fn type_number(self) -> Self::Output<Type> {
        self.r#type(Type::Number)
    }

    fn type_range(self) -> Self::Output<Type> {
        self.r#type(Type::Range)
    }

    fn type_color(self) -> Self::Output<Type> {
        self.r#type(Type::Color)
    }

    fn type_checkbox(self) -> Self::Output<Type> {
        self.r#type(Type::Checkbox)
    }

    fn type_radio(self) -> Self::Output<Type> {
        self.r#type(Type::Radio)
    }

    fn type_file(self) -> Self::Output<Type> {
        self.r#type(Type::File)
    }

    fn type_submit(self) -> Self::Output<Type> {
        self.r#type(Type::Submit)
    }

    fn type_image(self) -> Self::Output<Type> {
        self.r#type(Type::Image)
    }

    fn type_reset(self) -> Self::Output<Type> {
        self.r#type(Type::Reset)
    }

    fn type_button(self) -> Self::Output<Type> {
        self.r#type(Type::Button)
    }

    /// Value of the form control
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self::Output<Width> {
        self.with_attribute(Width(width))
    }

    fn on_input<E>(self, event: impl FnOnce(InputEvent) -> E) -> Self::Output<OnInput>
    where
        E: ::serde::Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.with_attribute(OnInput(Box::new(move || {
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

    fn on_change<E>(self, event: impl FnOnce(InputEvent) -> E) -> Self::Output<OnChange>
    where
        E: ::serde::Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.with_attribute(OnChange(Box::new(move || {
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

    fn on_search<E>(self, event: impl FnOnce(InputEvent) -> E) -> Self::Output<OnSearch>
    where
        E: ::serde::Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.with_attribute(OnSearch(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_search event",
                    err,
                })
                .map(|json| (hash, json))
        })))
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

pub struct OnSearch(pub Box<SerializeEventFn>);

impl Attributes for OnSearch {
    fn render(self, r: &mut crate::render::ElementRenderer) -> Result<(), crate::Error> {
        // TODO: directly write into el?
        let (id, payload) = &(self.0)()?;
        r.attribute("cabin-search", id)
            .map_err(crate::error::InternalError::from)?;
        r.attribute("cabin-search-payload", payload)
            .map_err(crate::error::InternalError::from)?;

        Ok(())
    }
}
