use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::anchor::{Href, ReferrerPolicy};
use super::button::Disabled;
use super::common::Common;
use super::global::Global;
use super::img::Sizes;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::list::SpaceSeparated;
use crate::html::{Aria, Html};

/// A `link` element allows to link to other resources.
pub fn link() -> Html<marker::Link, (), ()> {
    Html::new("link", (), ())
}

pub mod marker {
    pub struct Link;
}

impl<A: Attributes, V: 'static> Link for Html<marker::Link, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Link, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Link, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Link, A, V> {}

/// A `link` element allows to link to other resources.
pub trait Link: WithAttribute {
    //// Address of the hyperlink.
    fn href(self, href: impl Into<Cow<'static, str>>) -> Self::Output<Href> {
        self.with_attribute(Href(href.into()))
    }

    /// Handling of crossorigin requests.
    fn cross_origin(self, cross_origin: CrossOrigin) -> Self::Output<CrossOrigin> {
        self.with_attribute(cross_origin)
    }

    /// Relationship between the document and the linked resource.
    fn rel(self, rel: impl Into<SpaceSeparated<Rel>>) -> Self::Output<RelList> {
        self.with_attribute(RelList(rel.into()))
    }

    /// Appends a [Rel] to the link.
    fn append_rel(mut self, rel: Rel) -> Self::Output<RelList> {
        let rel_list = if let Some(list) = self.get_attribute_mut::<RelList>() {
            RelList(
                match std::mem::replace(&mut list.0, SpaceSeparated::Single(Rel::Alternate)) {
                    SpaceSeparated::Single(existing) => {
                        SpaceSeparated::List([existing, rel].into())
                    }
                    SpaceSeparated::List(mut list) => {
                        list.insert(rel);
                        SpaceSeparated::List(list)
                    }
                },
            )
        } else {
            RelList(SpaceSeparated::Single(rel))
        };
        self.with_attribute(rel_list)
    }

    fn r#as(self, r#as: As) -> Self::Output<As> {
        self.with_attribute(r#as)
    }

    /// The media the resource applies to.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self::Output<Media> {
        self.with_attribute(Media(media.into()))
    }

    /// Integrity metadata used in _Subresource Integrity_ checks.
    /// Must only be specified on links with [Rel::StyleSheet], [Rel::Preload], or
    /// [Rel::Modulepreload].
    fn integrity(self, integrity: impl Into<Cow<'static, str>>) -> Self::Output<Integrity> {
        self.with_attribute(Integrity(integrity.into()))
    }

    /// Hint the language of the linked resource.
    fn hreflang(self, hreflang: impl Into<Cow<'static, str>>) -> Self::Output<Hreflang> {
        self.with_attribute(Hreflang(hreflang.into()))
    }

    /// Hint for the type of the referenced resource.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self::Output<Type> {
        self.with_attribute(Type(r#type.into()))
    }

    /// Sizes of the icons ([Rel::Icon]).
    fn sizes(self, sizes: impl Into<Cow<'static, str>>) -> Self::Output<Sizes> {
        self.with_attribute(Sizes(sizes.into()))
    }

    /// Images to use in different situations.
    /// For [Rel::Preload] and [As::Image] only.
    fn image_srcset(self, image_srcset: impl Into<Cow<'static, str>>) -> Self::Output<ImageSrcset> {
        self.with_attribute(ImageSrcset(image_srcset.into()))
    }

    /// Image sizes for different page layouts.
    /// For [Rel::Preload] and [As::Image] only.
    fn image_sizes(self, image_sizes: impl Into<Cow<'static, str>>) -> Self::Output<ImageSizes> {
        self.with_attribute(ImageSizes(image_sizes.into()))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self::Output<ReferrerPolicy> {
        self.with_attribute(referrer_policy)
    }

    /// Indicate that the element is potentially render blocking.
    fn blocking(self) -> Self::Output<Blocking> {
        self.with_blocking(true)
    }

    /// Indicate that the element is potentially render blocking.
    fn with_blocking(self, blocking: bool) -> Self::Output<Blocking> {
        self.with_attribute(Blocking(blocking))
    }

    /// Whether the link is disabled.
    fn disabled(self) -> Self::Output<Disabled> {
        self.with_disabled(true)
    }

    /// Whether the link is disabled.
    fn with_disabled(self, disabled: bool) -> Self::Output<Disabled> {
        self.with_attribute(Disabled(disabled))
    }

    /// Sets the priority for fetches initiated by the element.
    fn fetch_priority(self, fetch_priority: FetchPriority) -> Self::Output<FetchPriority> {
        self.with_attribute(fetch_priority)
    }
}

/// Relationship between the document and the linked resource.
#[derive(Debug, Clone, Hash, Attribute)]
#[attribute(name = "rel")]
pub struct RelList(pub SpaceSeparated<Rel>);

/// The media the resource applies to.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Media(pub Cow<'static, str>);

/// Integrity metadata used in _Subresource Integrity_ checks.
/// Must only be specified on links with [Rel::StyleSheet], [Rel::Preload], or
/// [Rel::Modulepreload].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Integrity(pub Cow<'static, str>);

/// Hint the language of the linked resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Hreflang(pub Cow<'static, str>);

/// Hint for the type of the referenced resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Type(pub Cow<'static, str>);

/// Images to use in different situations.
/// For [Rel::Preload] and [As::Image] only.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ImageSrcset(pub Cow<'static, str>);

/// Image sizes for different page layouts.
/// For [Rel::Preload] and [As::Image] only.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct ImageSizes(pub Cow<'static, str>);

/// The referrer information send when following a hyperlink.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum CrossOrigin {
    /// Requests for the element will have their mode set to "cors" and their credientials mode
    /// set to "same-origin".
    Anonymous,

    /// Requests for the element will have their mode set to "cors" and their credientials mode
    /// set to "include".
    UseCredentials,
}

impl fmt::Display for CrossOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrossOrigin::Anonymous => f.write_str("anonymous"),
            CrossOrigin::UseCredentials => f.write_str("use-credentials"),
        }
    }
}

/// Relationship between the document and the linked resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rel {
    /// Alternate representation of the current document.
    Alternate,

    /// Preferred URL for the current document.
    Canonical,

    /// Link to the author of the current document.
    Author,

    /// Specifies that the user agent should preemptively perform DNS resolution for the target
    /// resource's origin.
    DnsPrefetch,

    /// Link to context-sensitive help.
    Help,

    /// An icon that represents the current document.
    Icon,

    /// An application manifest.
    Manifest,

    /// Specifies that the user agent must preemptively fetch the module script and store it in the
    /// document's module map for later evaluation.
    Modulepreload,

    /// Indicates that the main content of the current document is covered by the copyright license
    /// described by the referenced document.
    License,

    /// Indicates that the current document is a part of a series, and that the next document in
    /// the series is the referenced document.
    Next,

    /// Gives the address of the pingback server that handles pingbacks to the current document.
    Pingback,

    /// Specifies that the user agent should preemptively connect to the target resource's origin.
    Preconnect,

    /// Specifies that the user agent should preemptively fetch and cache the target resource as it
    /// is likely to be required for a followup navigation.
    Prefetch,

    /// Specifies that the user agent must preemptively fetch and cache the target resource for
    /// current navigation according to the potential destination given by the `as` attribute
    /// (and the `priority` associated with the corresponding destination).
    Preload,

    /// Indicates that the current document is a part of a series, and that the previous document
    /// in the series is the referenced document.
    Prev,

    /// Gives a link to a resource that can be used to search through the current document and its
    /// related pages.
    Search,

    /// Imports a style sheet.
    StyleSheet,
}

impl fmt::Display for Rel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Alternate => f.write_str("alternate"),
            Self::Canonical => f.write_str("canonical"),
            Self::Author => f.write_str("author"),
            Self::DnsPrefetch => f.write_str("dns-prefetch"),
            Self::Help => f.write_str("help"),
            Self::Icon => f.write_str("icon"),
            Self::Manifest => f.write_str("manifest"),
            Self::License => f.write_str("license"),
            Self::Modulepreload => f.write_str("modulepreload"),
            Self::Next => f.write_str("next"),
            Self::Pingback => f.write_str("pingback"),
            Self::Preconnect => f.write_str("preconnect"),
            Self::Prefetch => f.write_str("prefetch"),
            Self::Preload => f.write_str("preload"),
            Self::Prev => f.write_str("prev"),
            Self::Search => f.write_str("search"),
            Self::StyleSheet => f.write_str("stylesheet"),
        }
    }
}

/// Type of resource being preloaded.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum As {
    Fetch,
    Audio,
    AudioWorklet,
    Document,
    Embed,
    Font,
    Frame,
    IFrame,
    Image,
    Manifest,
    Object,
    PaintWorklet,
    Report,
    Script,
    ServiceWorker,
    SharedWorker,
    Style,
    Track,
    Video,
    WebIdentity,
    Worker,
    Xslt,
}

impl fmt::Display for As {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch => f.write_str("fetch"),
            Self::Audio => f.write_str("audio"),
            Self::AudioWorklet => f.write_str("audioworklet"),
            Self::Document => f.write_str("document"),
            Self::Embed => f.write_str("embed"),
            Self::Font => f.write_str("font"),
            Self::Frame => f.write_str("frame"),
            Self::IFrame => f.write_str("iframe"),
            Self::Image => f.write_str("image"),
            Self::Manifest => f.write_str("manifest"),
            Self::Object => f.write_str("object"),
            Self::PaintWorklet => f.write_str("paintworklet"),
            Self::Report => f.write_str("report"),
            Self::Script => f.write_str("script"),
            Self::ServiceWorker => f.write_str("serviceworker"),
            Self::SharedWorker => f.write_str("sharedworker"),
            Self::Style => f.write_str("style"),
            Self::Track => f.write_str("track"),
            Self::Video => f.write_str("video"),
            Self::WebIdentity => f.write_str("webidentity"),
            Self::Worker => f.write_str("worker"),
            Self::Xslt => f.write_str("xslt"),
        }
    }
}

/// The element is potentially render-blocking.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Blocking(pub bool);

impl fmt::Display for Blocking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 {
            f.write_str("render")?;
        }
        Ok(())
    }
}

/// Sets the priority for fetches initiated by the element.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum FetchPriority {
    /// Signals automatic determination of fetch priority relative to other resources with the
    /// same destination.
    #[default]
    Auto,

    /// Signals a high-priority fetch relative to other resources with the same destination.
    High,

    /// Signals a low-priority fetch relative to other resources with the same destination.
    Low,
}

impl fmt::Display for FetchPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::High => f.write_str("high"),
            Self::Low => f.write_str("low"),
        }
    }
}
