use std::borrow::Cow;
use std::fmt;

use cabin_macros::Element;

use crate::html::list::SpaceSeparated;

use super::anchor::ReferrerPolicy;

/// A `link` element allows to link to other resources.
#[derive(Default, Element)]
#[element(void)]
pub struct Link {
    /// Address of the hyperlink.
    href: Option<Cow<'static, str>>,

    /// Handling of crossorigin requests.
    #[element(attribute_name = "crossorigin")]
    cross_origin: Option<CrossOrigin>,

    /// Relationship between the document and the linked resource.
    rel: Option<SpaceSeparated<Rel>>,

    // Potential destination for a preload request ([Rel::Preload], [Rel::Modulepreload]).
    r#as: Option<As>,

    /// The media the resource applies to.
    media: Option<Cow<'static, str>>,

    /// Integrity metadata used in _Subresource Integrity_ checks.
    /// Must only be specified on links with [Rel::StyleSheet], [Rel::Preload], or
    /// [Rel::Modulepreload].
    integrity: Option<Cow<'static, str>>,

    /// Hint the language of the linked resource.
    hreflang: Option<Cow<'static, str>>,

    /// Hint for the type of the referenced resource.
    r#type: Option<Cow<'static, str>>,

    /// Sizes of the icons ([Rel::Icon]).
    sizes: Option<Cow<'static, str>>,

    /// Images to use in different situations.
    /// For [Rel::Preload] and [As::Image] only.
    #[element(attribute_name = "imageSrcset")]
    image_srcset: Option<Cow<'static, str>>,

    /// Image sizes for different page layouts.
    /// For [Rel::Preload] and [As::Image] only.
    #[element(attribute_name = "imageSizes")]
    image_sizes: Option<Cow<'static, str>>,

    /// How much referrer information to send.
    #[element(attribute_name = "referrerpolicy")]
    referrer_policy: ReferrerPolicy,

    #[element(skip)]
    blocking: Option<RenderBlocking>,

    /// Whether the link is disabled.
    disabled: bool,

    /// Sets the priority for fetches initiated by the element.
    #[element(attribute_name = "fetchpriority")]
    fetch_priority: FetchPriority,
}

impl LinkElement {
    /// Appends a [Rel] to the link.
    pub fn append_rel(mut self, rel: Rel) -> Self {
        self.kind.rel = match self.kind.rel.take() {
            Some(SpaceSeparated::Single(existing)) => {
                Some(SpaceSeparated::List([existing, rel].into()))
            }
            Some(SpaceSeparated::List(mut list)) => {
                list.insert(rel);
                Some(SpaceSeparated::List(list))
            }
            None => Some(SpaceSeparated::Single(rel)),
        };
        self
    }

    /// Indicate that the element is potentially render blocking.
    pub fn blocking(mut self) -> Self {
        self.kind.blocking = Some(RenderBlocking);
        self
    }
}

/// The referrer information send when following a hyperlink.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            Rel::Alternate => f.write_str("alternate"),
            Rel::Canonical => f.write_str("canonical"),
            Rel::Author => f.write_str("author"),
            Rel::DnsPrefetch => f.write_str("dns-prefetch"),
            Rel::Help => f.write_str("help"),
            Rel::Icon => f.write_str("icon"),
            Rel::Manifest => f.write_str("manifest"),
            Rel::License => f.write_str("license"),
            Rel::Modulepreload => f.write_str("modulepreload"),
            Rel::Next => f.write_str("next"),
            Rel::Pingback => f.write_str("pingback"),
            Rel::Preconnect => f.write_str("preconnect"),
            Rel::Prefetch => f.write_str("prefetch"),
            Rel::Preload => f.write_str("preload"),
            Rel::Prev => f.write_str("prev"),
            Rel::Search => f.write_str("search"),
            Rel::StyleSheet => f.write_str("stylesheet"),
        }
    }
}

/// Type of resource being preloaded.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            As::Fetch => f.write_str("fetch"),
            As::Audio => f.write_str("audio"),
            As::AudioWorklet => f.write_str("audioworklet"),
            As::Document => f.write_str("document"),
            As::Embed => f.write_str("embed"),
            As::Font => f.write_str("font"),
            As::Frame => f.write_str("frame"),
            As::IFrame => f.write_str("iframe"),
            As::Image => f.write_str("image"),
            As::Manifest => f.write_str("manifest"),
            As::Object => f.write_str("object"),
            As::PaintWorklet => f.write_str("paintworklet"),
            As::Report => f.write_str("report"),
            As::Script => f.write_str("script"),
            As::ServiceWorker => f.write_str("serviceworker"),
            As::SharedWorker => f.write_str("sharedworker"),
            As::Style => f.write_str("style"),
            As::Track => f.write_str("track"),
            As::Video => f.write_str("video"),
            As::WebIdentity => f.write_str("webidentity"),
            As::Worker => f.write_str("worker"),
            As::Xslt => f.write_str("xslt"),
        }
    }
}

/// The element is potentially render-blocking.
#[derive(Hash)]
pub(super) struct RenderBlocking;

impl fmt::Display for RenderBlocking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("render")
    }
}

/// The priority for fetches initiated by an element.
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
            FetchPriority::Auto => f.write_str("auto"),
            FetchPriority::High => f.write_str("high"),
            FetchPriority::Low => f.write_str("low"),
        }
    }
}
