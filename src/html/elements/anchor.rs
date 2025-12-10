use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use super::link::Type;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::list::SpaceSeparated;
use crate::html::{Aria, Html};

/// An `a` element that – if `href` is specified – creates a hyperlink to anything a URL can
/// address.
pub fn a(content: impl View) -> Html<marker::Anchor, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("a", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! a {
        ($($x:tt)*) => {
            $crate::html::elements::anchor::a($crate::view![$($x)*])
        }
    }

    pub use a;
}

pub use macros::a;

pub mod marker {
    pub struct Anchor;
}

impl<A: Attributes> Anchor for Html<marker::Anchor, A> {}
impl<A: Attributes> Common for Html<marker::Anchor, A> {}
impl<A: Attributes> Global for Html<marker::Anchor, A> {}
impl<A: Attributes> Aria for Html<marker::Anchor, A> {}

/// An `a` element that – if `href` is specified – creates a hyperlink to anything a URL can
/// address.
pub trait Anchor: WithAttribute {
    /// Address of the hyperlink.
    fn href(self, href: impl Into<Cow<'static, str>>) -> Self::Output<Href> {
        self.with_attribute(Href(href.into()))
    }

    /// The _browsing context_ the link should be opened in.
    fn target(self, target: impl Into<Cow<'static, str>>) -> Self::Output<Target> {
        self.with_attribute(Target(target.into()))
    }

    /// Try to open the link in a new tab.
    fn target_blank(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_blank")))
    }

    /// Open the link in the parent browsing context.
    fn target_parent(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_parent")))
    }

    /// Open the link in the topmost browsing context.
    fn target_top(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_top")))
    }

    /// Treat the linked URL as a download with the specified filename.
    fn download_filename(self, download: impl Into<Cow<'static, str>>) -> Self::Output<Download> {
        self.with_attribute(Download(download.into()))
    }

    /// Treat the linked URL as a download and let the browser suggest a filename.
    fn download(self) -> Self::Output<Download> {
        self.with_attribute(Download(Cow::Borrowed("")))
    }

    /// A space-separated list of URLs the browser will send POST requests (with the body PING)
    /// when the link is followed (typically used for tracking).
    fn ping(self, ping: impl Into<Cow<'static, str>>) -> Self::Output<Ping> {
        self.with_attribute(Ping(ping.into()))
    }

    /// Relationship between the location in the document containing the hyperlink and the
    /// destination resource.
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

    /// Hint the language of the linked resource.
    fn hreflang(self, hreflang: impl Into<Cow<'static, str>>) -> Self::Output<Hreflang> {
        self.with_attribute(Hreflang(hreflang.into()))
    }

    /// Hint for the type of the referenced resource.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self::Output<Type> {
        self.with_attribute(Type(r#type.into()))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self::Output<ReferrerPolicy> {
        self.with_attribute(referrer_policy)
    }
}

/// Address of the hyperlink.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Href(pub Cow<'static, str>);

/// The _browsing context_ the link should be opened in.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Target(pub Cow<'static, str>);

/// Treat the linked URL as a download with the specified filename.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Download(pub Cow<'static, str>);

/// A space-separated list of URLs the browser will send POST requests (with the body PING)
/// when the link is followed (typically used for tracking).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Ping(pub Cow<'static, str>);

/// Relationship between the location in the document containing the hyperlink and the
/// destination resource.
#[derive(Debug, Clone, Hash, Attribute)]
#[attribute(name = "rel")]
pub struct RelList(pub SpaceSeparated<Rel>);

/// Hint the language of the linked resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Hreflang(pub Cow<'static, str>);

/// Relationship between the document and the linked resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rel {
    /// Alternate representation of the current document.
    Alternate,

    /// Link to the author of the current document.
    Author,

    /// Gives the permalink for the nearest ancestor section.
    Bookmark,

    /// Indicates that the referenced document is not part of the same site as the current
    /// document.
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
            Self::Alternate => f.write_str("alternate"),
            Self::Author => f.write_str("author"),
            Self::Bookmark => f.write_str("bookmark"),
            Self::External => f.write_str("external"),
            Self::Help => f.write_str("help"),
            Self::License => f.write_str("license"),
            Self::Next => f.write_str("next"),
            Self::NoFollow => f.write_str("nofollow"),
            Self::NoOpener => f.write_str("noopener"),
            Self::NoReferrer => f.write_str("noreferrer"),
            Self::Opener => f.write_str("opener"),
            Self::Prev => f.write_str("prev"),
            Self::Search => f.write_str("search"),
            Self::Tag => f.write_str("tag"),
        }
    }
}

/// The referrer information send when following a hyperlink.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
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
            Self::NoReferrer => f.write_str("no-referrer"),
            Self::NoReferrerWhenDowngrade => f.write_str("no-referrer-when-downgrade"),
            Self::SameOrigin => f.write_str("same-origin"),
            Self::Origin => f.write_str("origin"),
            Self::StrictOrigin => f.write_str("strict-origin"),
            Self::OriginWhenCrossOrigin => f.write_str("origin-when-cross-origin"),
            Self::StrictOriginWhenCrossOrigin => f.write_str("strict-origin-when-cross-origin"),
            Self::UnsafeUrl => f.write_str("unsafe-url"),
        }
    }
}
