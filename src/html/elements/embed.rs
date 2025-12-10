use std::borrow::Cow;

use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use super::link::Type;
use super::script::Src;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The <embed< element provides an integration point for an external application or interactive
/// content.
pub fn embed() -> Html<marker::Embed, ()> {
    Html::new("embed", (), ()).into_void_element()
}

pub mod marker {
    pub struct Embed;
}

impl<A: Attributes> Embed for Html<marker::Embed, A> {}
impl<A: Attributes> Common for Html<marker::Embed, A> {}
impl<A: Attributes> Global for Html<marker::Embed, A> {}
impl<A: Attributes> Aria for Html<marker::Embed, A> {}

/// The <embed< element provides an integration point for an external application or interactive
/// content.
pub trait Embed: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Src> {
        self.with_attribute(Src(src.into()))
    }

    ///  Type of embedded resource
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self::Output<Type> {
        self.with_attribute(Type(r#type.into()))
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
