use std::borrow::Cow;

use super::global::Global;
use super::img::{Sizes, SrcSet};
use super::input::{Height, Width};
use super::link::Type;
use super::meta::Media;
use super::script::Src;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::void_element::VoidElement;

impl Context {
    /// The `source` element allows authors to specify multiple alternative source sets for
    /// [super::img] elements or multiple alternative media resources for media elements. It
    /// does not represent anything on its own.
    pub fn source(&self) -> VoidElement<marker::Source> {
        VoidElement::new(self.acquire_renderer(), "source")
    }
}

pub mod marker {
    pub struct Source;
}

impl Source for VoidElement<marker::Source> {}
impl Global for VoidElement<marker::Source> {}

/// An `source` element represents an image.
pub trait Source: WithAttribute {
    /// Type of embedded resource.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Type(r#type.into()))
    }

    /// Applicable media.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Media(media.into()))
    }

    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Src(src.into()))
    }

    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
    fn src_set(self, src_set: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(SrcSet(src_set.into()))
    }

    /// Image sizes for different page layouts.
    fn sizes(self, sizes: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Sizes(sizes.into()))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self {
        self.with_attribute(Width(width))
    }
}
