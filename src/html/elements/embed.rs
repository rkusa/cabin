use std::borrow::Cow;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use super::link::Type;
use super::script::Src;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::void_element::VoidElement;

impl Context {
    /// The `embed` element provides an integration point for an external application or interactive
    /// content.
    pub fn embed(&self) -> VoidElement<marker::Embed> {
        VoidElement::new(self.acquire_renderer(), "embed")
    }
}

pub mod marker {
    pub struct Embed;
}

impl Embed for VoidElement<marker::Embed> {}
impl Common for VoidElement<marker::Embed> {}
impl Global for VoidElement<marker::Embed> {}
impl Aria for VoidElement<marker::Embed> {}

/// The <embed< element provides an integration point for an external application or interactive
/// content.
pub trait Embed: WithAttribute {
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
