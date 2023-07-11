use std::borrow::Cow;
use std::fmt;

use cabin_macros::Element;

use crate::html::list::SpaceSeparated;

/// An `a` element that – if `href` is specified – creates a hyperlink to anything a URL can
/// address.
#[derive(Default, Element)]
#[element(tag_name = "a")]
pub struct Anchor {
    /// Address of the hyperlink.
    href: Option<Cow<'static, str>>,

    /// The _browsing context_ the link should be opened in.
    target: Option<Cow<'static, str>>,

    /// Treat the linked URL as a download with the specified filename.
    #[element(method_name = "download_filename")]
    download: Option<Cow<'static, str>>,

    /// A space-separated list of URLs the browser will send POST requests (with the body PING)
    /// when the link is followed (typically used for tracking).
    ping: Option<Cow<'static, str>>,

    /// Relationship between the location in the document containing the hyperlink and the
    /// destination resource.
    rel: Option<SpaceSeparated<Rel>>,

    /// Hint the language of the linked resource.
    hreflang: Option<Cow<'static, str>>,

    /// Hint for the type of the referenced resource.
    #[element(attribute_name = "type")]
    r#type: Option<Cow<'static, str>>,

    /// How much referrer information to send.
    #[element(attribute_name = "referrerpolicy")]
    referrer_policy: ReferrerPolicy,
}

impl<Ext> AnchorElement<Ext> {
    /// Try to open the link in a new tab.
    pub fn target_blank(mut self) -> Self {
        self.base.target = Some(Cow::Borrowed("_blank"));
        self
    }

    /// Open the link in the parent browsing context.
    pub fn target_parent(mut self) -> Self {
        self.base.target = Some(Cow::Borrowed("_parent"));
        self
    }

    /// Open the link in the topmost browsing context.
    pub fn target_top(mut self) -> Self {
        self.base.target = Some(Cow::Borrowed("_top"));
        self
    }

    /// Treat the linked URL as a download and let the browser suggest a filename.
    pub fn download(mut self) -> Self {
        self.base.download = Some(Cow::Borrowed(""));
        self
    }

    /// Appends a [Rel] to the link.
    pub fn append_rel(mut self, rel: Rel) -> Self {
        self.base.rel = match self.base.rel.take() {
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
}

/// Relationship between the document and the linked resource.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rel {
    /// Alternate representation of the current document.
    Alternate,

    /// Link to the author of the current document.
    Author,

    /// Gives the permalink for the nearest ancestor section.
    Bookmark,

    /// Indicates that the referenced document is not part of the same site as the current document.
    External,

    /// Link to context-sensitive help.
    Help,

    /// Indicates that the main content of the current document is covered by the copyright license
    /// described by the referenced document.
    License,

    /// Indicates that the current document is a part of a series, and that the next document in
    /// the series is the referenced document.
    Next,

    /// Indicates that the current document's original author or publisher does not endorse the
    /// referenced document.
    NoFollow,

    /// Instruct the browser to navigate to the target resource without granting the new browsing
    /// context access to the document that opened it
    NoOpener,

    /// No `Referer` (sic) header will be included. Additionally, has the same effect as
    /// [Self::NoOpener].
    NoReferrer,

    /// Opposit of [Self::NoOpener].
    Opener,

    /// Indicates that the current document is a part of a series, and that the previous document
    /// in the series is the referenced document.
    Prev,

    /// Gives a link to a resource that can be used to search through the current document and its
    /// related pages.
    Search,

    /// Gives a tag (identified by the given address) that applies to the current document.
    Tag,
}

impl fmt::Display for Rel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rel::Alternate => f.write_str("alternate"),
            Rel::Author => f.write_str("author"),
            Rel::Bookmark => f.write_str("bookmark"),
            Rel::External => f.write_str("external"),
            Rel::Help => f.write_str("help"),
            Rel::License => f.write_str("license"),
            Rel::Next => f.write_str("next"),
            Rel::NoFollow => f.write_str("nofollow"),
            Rel::NoOpener => f.write_str("noopener"),
            Rel::NoReferrer => f.write_str("noreferrer"),
            Rel::Opener => f.write_str("opener"),
            Rel::Prev => f.write_str("prev"),
            Rel::Search => f.write_str("search"),
            Rel::Tag => f.write_str("tag"),
        }
    }
}

/// The referrer information send when following a hyperlink.
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ReferrerPolicy {
    /// No referrer information always.
    NoReferrer,

    /// No referrer information from HTTPS to HTTP.
    /// Full URL for HTTPS to HTTPS and HTTP to HTTP.
    NoReferrerWhenDowngrade,

    /// No referrer information for cross-origin requests.
    /// Full URL for same-origin requests.
    SameOrigin,

    /// Origin only always.
    Origin,

    /// No referrer information from HTTPS to HTTP.
    /// Origin only for HTTPS to HTTPS and HTTP to HTTP.
    StrictOrigin,

    /// Origin only for cross-origin requests.
    /// Full URL for same-origin requests.
    OriginWhenCrossOrigin,

    /// No referrer information from HTTPS to HTTP.
    /// Origin only for HTTPS to HTTPS and HTTP to HTTP.
    /// Full URL for same-origin requests.
    #[default]
    StrictOriginWhenCrossOrigin,

    /// Full URL always.
    UnsafeUrl,
}

impl fmt::Display for ReferrerPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferrerPolicy::NoReferrer => f.write_str("no-referrer"),
            ReferrerPolicy::NoReferrerWhenDowngrade => f.write_str("no-referrer-when-downgrade"),
            ReferrerPolicy::SameOrigin => f.write_str("same-origin"),
            ReferrerPolicy::Origin => f.write_str("origin"),
            ReferrerPolicy::StrictOrigin => f.write_str("strict-origin"),
            ReferrerPolicy::OriginWhenCrossOrigin => f.write_str("origin-when-cross-origin"),
            ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                f.write_str("strict-origin-when-cross-origin")
            }
            ReferrerPolicy::UnsafeUrl => f.write_str("unsafe-url"),
        }
    }
}
