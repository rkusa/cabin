use std::borrow::Cow;

use super::global::Global;
use super::img::{Sizes, SrcSet};
use super::input::{Height, Width};
use super::link::Type;
use super::meta::Media;
use super::script::Src;
use crate::html::Html;
use crate::html::attributes::{Attributes, WithAttribute};

/// The `source` element allows authors to specify multiple alternative source sets for [super::img]
/// elements or multiple alternative media resources for media elements. It does not represent
/// anything on its own.
pub fn source() -> Html<marker::Source, (), ()> {
    Html::new("source", (), ()).into_void_element()
}

pub mod marker {
    pub struct Source;
}

impl<A: Attributes, V: 'static> Source for Html<marker::Source, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Source, A, V> {}

/// An `source` element represents an image.
pub trait Source: WithAttribute {
    /// Type of embedded resource.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self::Output<Type> {
        self.with_attribute(Type(r#type.into()))
    }

    /// Applicable media.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self::Output<Media> {
        self.with_attribute(Media(media.into()))
    }

    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Src> {
        self.with_attribute(Src(src.into()))
    }

    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
    fn src_set(self, src_set: impl Into<Cow<'static, str>>) -> Self::Output<SrcSet> {
        self.with_attribute(SrcSet(src_set.into()))
    }

    /// Image sizes for different page layouts.
    fn sizes(self, sizes: impl Into<Cow<'static, str>>) -> Self::Output<Sizes> {
        self.with_attribute(Sizes(sizes.into()))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self::Output<Height> {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self::Output<Width> {
        self.with_attribute(Width(width))
    }
}
