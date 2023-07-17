use std::borrow::Cow;
use std::fmt;

use cabin_macros::{element, Attribute};

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

// TODO
#[element]
pub trait Form: Common + Global + Aria {
    /// URL to use for form submission.
    fn action(self, action: impl Into<Cow<'static, str>>) -> impl Form {
        self.with(Action(action.into()))
    }

    /// Variant used for form submission.
    fn method(self, method: Method) -> impl Form {
        self.with(method)
    }

    /// Set the form's method to `get`.
    fn method_get(self) -> impl Form {
        self.method(Method::Get)
    }

    /// Set the form's method to `post`.
    fn method_post(self) -> impl Form {
        self.method(Method::Post)
    }

    fn on_submit<E>(self) -> impl Form
    where
        E: 'static,
    {
        use std::hash::{Hash, Hasher};

        let mut hasher = twox_hash::XxHash32::default();
        std::any::TypeId::of::<E>().hash(&mut hasher);
        let hash = hasher.finish() as u32;

        self.with(OnSubmit(hash))
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

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
#[attribute(name = "cabin-submit")]
pub struct OnSubmit(pub u32);
