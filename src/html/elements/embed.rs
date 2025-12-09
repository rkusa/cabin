use std::borrow::Cow;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use super::link::Type;
use super::script::Src;
use crate::attribute::WithAttribute;
use crate::void_element::{VoidElement, VoidElementProxy};

/// The `embed` element provides an integration point for an external application or interactive
/// content.
pub fn embed() -> VoidElement<marker::Embed> {
    VoidElement::new("embed")
}

pub mod marker {
    pub struct Embed;
}

impl<P> Embed<marker::Embed> for P where P: VoidElementProxy<marker::Embed> {}
impl<P> Common<marker::Embed> for P where P: VoidElementProxy<marker::Embed> {}
impl<P> Global<marker::Embed> for P where P: VoidElementProxy<marker::Embed> {}
impl<P> Aria<marker::Embed> for P where P: VoidElementProxy<marker::Embed> {}

/// The <embed< element provides an integration point for an external application or interactive
/// content.
pub trait Embed<T>: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Src(src.into()))
    }

    ///  Type of embedded resource
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Type(r#type.into()))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self {
        self.with_attribute(Width(width))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self {
        self.with_attribute(Height(height))
    }
}
