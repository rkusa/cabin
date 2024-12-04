use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::anchor::ReferrerPolicy;
use super::common::Common;
use super::global::Global;
use super::iframe::Loading;
use super::input::{Height, Width};
use super::link::{CrossOrigin, FetchPriority};
use super::script::Src;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// An `img` element represents an image.
pub fn img() -> Html<marker::Img, (), ()> {
    Html::new("img", (), ()).into_void_element()
}

pub mod marker {
    pub struct Img;
}

impl<A: Attributes, V: 'static> Img for Html<marker::Img, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Img, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Img, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Img, A, V> {}

/// An `img` element represents an image.
pub trait Img: WithAttribute {
    /// Replacement text for use when images are not available.
    fn alt(self, alt: impl Into<Cow<'static, str>>) -> Self::Output<Alt> {
        self.with_attribute(Alt(alt.into()))
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

    /// Handling of crossorigin requests.
    fn cross_origin(self, cross_origin: CrossOrigin) -> Self::Output<CrossOrigin> {
        self.with_attribute(cross_origin)
    }

    /// Name of image map to use.
    fn use_map(self, use_map: impl Into<Cow<'static, str>>) -> Self::Output<UseMap> {
        self.with_attribute(UseMap(use_map.into()))
    }

    /// Whether the image is a server-side image map.
    #[allow(clippy::wrong_self_convention)]
    fn is_map(self) -> Self::Output<IsMap> {
        self.with_is_map(true)
    }

    /// Whether the image is a server-side image map.
    fn with_is_map(self, is_map: bool) -> Self::Output<IsMap> {
        self.with_attribute(IsMap(is_map))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self::Output<Height> {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self::Output<Width> {
        self.with_attribute(Width(width))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self::Output<ReferrerPolicy> {
        self.with_attribute(referrer_policy)
    }

    /// Decoding hint to use when processing this image for presentation.
    fn decoding(self, decoding: Decoding) -> Self::Output<Decoding> {
        self.with_attribute(decoding)
    }

    /// Used when determining loading deferral.
    fn loading(self, loading: Loading) -> Self::Output<Loading> {
        self.with_attribute(loading)
    }

    /// Sets the priority for fetches initiated by the element.
    fn fetch_priority(self, fetch_priority: FetchPriority) -> Self::Output<FetchPriority> {
        self.with_attribute(fetch_priority)
    }
}

/// Replacement text for use when images are not available.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Alt(pub Cow<'static, str>);

/// Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct SrcSet(pub Cow<'static, str>);

/// Image sizes for different page layouts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Sizes(pub Cow<'static, str>);

/// Name of image map to use.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct UseMap(pub Cow<'static, str>);

/// Whether the image is a server-side image map.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct IsMap(pub bool);

/// Decoding hint to use when processing this image for presentation.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Decoding {
    /// Indicates a preference to decode this image synchronously for atomic presentation with
    /// other content.
    Sync,

    /// Indicates a preference to decode this image asynchronously to avoid delaying presentation
    /// of other content.
    Async,

    /// Indicates no preference in decoding mode.
    #[default]
    Auto,
}

impl fmt::Display for Decoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Decoding::Sync => "sync",
            Decoding::Async => "async",
            Decoding::Auto => "auto",
        })
    }
}
