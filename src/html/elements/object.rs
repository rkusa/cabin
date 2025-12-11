use std::borrow::Cow;

use cabin_macros::Attribute;

use super::button::Form;
use super::common::Common;
use super::global::Global;
use super::iframe::Name;
use super::input::{Height, Width};
use super::link::Type;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `object` element can represent an external resource, which, depending on the type of the
/// resource, will either be treated as an image or as a child navigable.
#[crate::view_macro(cabin::html::elements::object)]
pub fn object(content: impl View) -> Html<marker::Object, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("object", (), content)
}

pub mod marker {
    pub struct Object;
}

impl<A: Attributes> Object for Html<marker::Object, A> {}
impl<A: Attributes> Common for Html<marker::Object, A> {}
impl<A: Attributes> Global for Html<marker::Object, A> {}
impl<A: Attributes> Aria for Html<marker::Object, A> {}

/// The `object` element can represent an external resource, which, depending on the type of the
/// resource, will either be treated as an image or as a child navigable.
pub trait Object: WithAttribute {
    /// Address of the resource.
    fn data(self, data: impl Into<Cow<'static, str>>) -> Self::Output<Data> {
        self.with_attribute(Data(data.into()))
    }

    /// Type of embedded resource.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self::Output<Type> {
        self.with_attribute(Type(r#type.into()))
    }

    /// Name of content navigable.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }

    /// Associates the element with a [super::form] element.
    fn form(self, form: impl Into<Cow<'static, str>>) -> Self::Output<Form> {
        self.with_attribute(Form(form.into()))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self::Output<Width> {
        self.with_attribute(Width(width))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self::Output<Height> {
        self.with_attribute(Height(height))
    }
}

/// Address of the resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Data(pub Cow<'static, str>);
