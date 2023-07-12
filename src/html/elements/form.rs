use std::borrow::Cow;
use std::fmt;

use cabin_macros::Element;

use crate::html::attributes::Attributes;

// TODO
#[derive(Default, Element)]
pub struct FormAttributes {
    // accept-charset — Character encodings to use for form submission
    /// URL to use for form submission.
    action: Option<Cow<'static, str>>,
    // autocomplete — Default setting for autofill feature for controls in the form
    // enctype — Entry list encoding type to use for form submission
    /// Variant used for form submission.
    method: Option<Method>,
    // name — Name of form to use in the document.forms API
    // novalidate — Bypass form control validation for form submission
    // target — Navigable for form submission
    // rel
}

pub trait FormExt: AsMut<FormAttributes> + Sized {
    /// Set the form's method to `get`.
    fn method_get(mut self) -> Self {
        self.as_mut().method = Some(Method::Get);
        self
    }

    /// Set the form's method to `post`.
    fn method_post(mut self) -> Self {
        self.as_mut().method = Some(Method::Post);
        self
    }
}

impl FormExt for Attributes<FormAttributes> {}

/// Variant used for form submission.
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            Method::Get => f.write_str("get"),
            Method::Post => f.write_str("post"),
            Method::Dialog => f.write_str("dialog"),
        }
    }
}
