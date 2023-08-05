use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

pub fn form(content: impl View) -> Html<marker::Form, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("form", (), content)
}

pub mod marker {
    pub struct Form;
}

impl<A: Attributes, V: 'static> Form for Html<marker::Form, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Form, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Form, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Form, A, V> {}

// TODO
pub trait Form: WithAttribute {
    /// URL to use for form submission.
    fn action(self, action: impl Into<Cow<'static, str>>) -> Self::Output<Action> {
        self.with_attribute(Action(action.into()))
    }

    /// Variant used for form submission.
    fn method(self, method: Method) -> Self::Output<Method> {
        self.with_attribute(method)
    }

    /// Set the form's method to `get`.
    fn method_get(self) -> Self::Output<Method> {
        self.method(Method::Get)
    }

    /// Set the form's method to `post`.
    fn method_post(self) -> Self::Output<Method> {
        self.method(Method::Post)
    }

    fn on_submit<E>(self) -> Self::Output<OnSubmit>
    where
        E: 'static,
    {
        use std::hash::{Hash, Hasher};

        let mut hasher = twox_hash::XxHash32::default();
        std::any::TypeId::of::<E>().hash(&mut hasher);
        let hash = hasher.finish() as u32;

        self.with_attribute(OnSubmit(hash))
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
