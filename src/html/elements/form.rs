use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::anchor::Target;
use super::aria::Aria;
use super::button::Name;
use super::common::Common;
use super::global::Global;
use super::input::AutoComplete;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;
use crate::event::Event;
use crate::html::events::CustomEvent;
use crate::html::list::SpaceSeparated;

impl Context {
    /// The `form` element represents a hyperlink that can be manipulated through a collection of
    /// form-associated elements, some of which can represent editable values that can be submitted
    /// to a server for processing.
    pub fn form(&self) -> Element<'_, marker::Form> {
        Element::new(self, "form")
    }
}

pub mod marker {
    pub struct Form;
}

impl<'v> Form for Element<'v, marker::Form> {}
impl<'v> Common for Element<'v, marker::Form> {}
impl<'v> Global for Element<'v, marker::Form> {}
impl<'v> Aria for Element<'v, marker::Form> {}

/// The `form` element represents a hyperlink that can be manipulated through a collection of
/// form-associated elements, some of which can represent editable values that can be submitted to a
/// server for processing.
pub trait Form: WithAttribute {
    /// Character encodings to use for form submission.
    fn accept_charset(self, accept_charset: AcceptCharset) -> Self {
        self.with_attribute(accept_charset)
    }

    /// URL to use for form submission.
    fn action(self, action: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Action(action.into()))
    }

    /// Default setting for autofill feature for controls in the form.
    fn autocomplete(self, autocomplete: AutoComplete) -> Self {
        self.with_attribute(autocomplete)
    }

    /// Entry list encoding type to use for form submission.
    fn enctype(self, enctype: EncType) -> Self {
        self.with_attribute(enctype)
    }

    /// Variant used for form submission.
    fn method(self, method: Method) -> Self {
        self.with_attribute(method)
    }

    /// Set the form's method to `get`.
    fn method_get(self) -> Self {
        self.method(Method::Get)
    }

    /// Set the form's method to `post`.
    fn method_post(self) -> Self {
        self.method(Method::Post)
    }

    /// Name of form to use in the `document.forms` API.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }

    /// Bypass form control validation for form submission.
    fn novalidate(self) -> Self {
        self.with_novalidate(true)
    }

    /// Bypass form control validation for form submission.
    fn with_novalidate(self, novalidate: bool) -> Self {
        self.with_attribute(NoValidate(novalidate))
    }

    /// The _browsing context_ the link should be opened in.
    fn target(self, target: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Target(target.into()))
    }

    /// Try to open the link in a new tab.
    fn target_blank(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_blank")))
    }

    /// Open the link in the parent browsing context.
    fn target_parent(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_parent")))
    }

    /// Open the link in the topmost browsing context.
    fn target_top(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_top")))
    }

    /// Relationship between the location in the document containing the hyperlink and the
    /// destination resource.
    fn rel(self, rel: impl Into<SpaceSeparated<Rel>>) -> Self {
        self.with_attribute(RelList(rel.into()))
    }

    /// Intercept form submissions, and serialize the form elements into the `E` event, which can
    /// then be handled via [crate::scope::take_event] or [crate::scope::event].
    fn on_submit<E>(self) -> Self
    where
        E: Event + 'static,
    {
        self.with_attribute(OnSubmit(E::ID))
    }

    /// Intercept form submissions, and submit the provided `event`, which can then be handled via
    /// [crate::scope::take_event] or [crate::scope::event].
    fn on_submit_with<E>(self, event: E) -> Self
    where
        E: serde::Serialize + Event,
    {
        self.with_attribute(CustomEvent::new("submit", event))
    }
}

/// Character encodings to use for form submission.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum AcceptCharset {
    #[default]
    Utf8,
}

impl fmt::Display for AcceptCharset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Utf8 => "UTF-8",
        })
    }
}

/// Entry list encoding type to use for form submission.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum EncType {
    /// `application/x-www-form-urlencoded`
    #[default]
    FormUrlEncoded,
    /// `multipart/form-data`
    Multipart,
    /// `text/plain`
    Plain,
}

impl fmt::Display for EncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            EncType::FormUrlEncoded => "application/x-www-form-urlencoded",
            EncType::Multipart => "multipart/form-data",
            EncType::Plain => "text/plain",
        })
    }
}

/// URL to use for form submission.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Action(pub Cow<'static, str>);

/// Variant used for form submission.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Method {
    /// Submit as GET request.
    #[default]
    Get,

    /// Submit as POST request.
    Post,

    /// Close dialog form is content of.
    Dialog,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => f.write_str("get"),
            Self::Post => f.write_str("post"),
            Self::Dialog => f.write_str("dialog"),
        }
    }
}

/// Bypass form control validation for form submission.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct NoValidate(pub bool);

/// Relationship between the location in the document containing the hyperlink and the
/// destination resource.
#[derive(Debug, Clone, Hash, Attribute)]
#[attribute(name = "rel")]
pub struct RelList(pub SpaceSeparated<Rel>);

/// Relationship between the document and the linked resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rel {
    /// Indicates that the referenced document is not part of the same site as the current
    /// document.
    External,

    /// Link to context-sensitive help.
    Help,

    /// Indicates that the main content of the current document is covered by the copyright license
    /// described by the referenced document.
    License,

    /// Indicates that the current document is a part of a series, and that the next document in
    /// the series is the referenced document.
    Next,

    /// Indicates that the current document's original author or publisher does not endorse the
    /// referenced document.
    NoFollow,

    /// Instruct the browser to navigate to the target resource without granting the new browsing
    /// context access to the document that opened it
    NoOpener,

    /// No `Referer` (sic) header will be included. Additionally, has the same effect as
    /// [Self::NoOpener].
    NoReferrer,

    /// Opposit of [Self::NoOpener].
    Opener,

    /// Indicates that the current document is a part of a series, and that the previous document
    /// in the series is the referenced document.
    Prev,

    /// Gives a link to a resource that can be used to search through the current document and its
    /// related pages.
    Search,
}

impl fmt::Display for Rel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::External => f.write_str("external"),
            Self::Help => f.write_str("help"),
            Self::License => f.write_str("license"),
            Self::Next => f.write_str("next"),
            Self::NoFollow => f.write_str("nofollow"),
            Self::NoOpener => f.write_str("noopener"),
            Self::NoReferrer => f.write_str("noreferrer"),
            Self::Opener => f.write_str("opener"),
            Self::Prev => f.write_str("prev"),
            Self::Search => f.write_str("search"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "cabin-submit")]
pub struct OnSubmit(pub &'static str);
