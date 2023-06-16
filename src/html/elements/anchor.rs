use std::borrow::Cow;
use std::fmt;

use cabin_macros::Element;

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
    rel: Option<Cow<'static, str>>,

    /// Hint the language of the linked resource.
    hreflang: Option<Cow<'static, str>>,

    /// Hint for the type of the referenced resource.
    r#type: Option<Cow<'static, str>>,

    /// How much referrer information to send.
    #[element(attribute_name = "referrerpolicy")]
    referrer_policy: Option<ReferrerPolicy>,
}

/// The referrer information send when following a hyperlink.
#[derive(Hash)]
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
    StrictOriginWhenCrossOrigin,

    /// Full URL always.
    UnsafeUrl,
}

impl<V> AnchorElement<V> {
    /// Try to open the link in a new tab.
    pub fn target_blank(mut self) -> Self {
        self.kind.target = Some(Cow::Borrowed("_blank"));
        self
    }

    /// Open the link in the parent browsing context.
    pub fn target_parent(mut self) -> Self {
        self.kind.target = Some(Cow::Borrowed("_parent"));
        self
    }

    /// Open the link in the topmost browsing context.
    pub fn target_top(mut self) -> Self {
        self.kind.target = Some(Cow::Borrowed("_top"));
        self
    }

    /// Treat the linked URL as a download and let the browser suggest a filename.
    pub fn download(mut self) -> Self {
        self.kind.download = Some(Cow::Borrowed(""));
        self
    }
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
