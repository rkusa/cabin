use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// The `meta` element represents various kinds of metadata that cannot be expressed using the
/// [super::title::title], [super::base::base], [super::link::link], [super::style::style], and
/// [super::script::script] elements.
pub fn meta() -> Element<marker::Meta> {
    Element::new("meta")
}

pub mod marker {
    pub struct Meta;
}

impl Meta for Element<marker::Meta> {}
impl Common for Element<marker::Meta> {}
impl Global for Element<marker::Meta> {}
impl Aria for Element<marker::Meta> {}

/// The `meta` element represents various kinds of metadata that cannot be expressed using the
/// [super::title::title], [super::base::base], [super::link::link], [super::style::style], and
/// [super::script::script] elements.
pub trait Meta: WithAttribute {
    /// Metadata name.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }

    /// Pragma directive.
    fn http_equiv(self, http_equiv: HttpEquiv) -> Self {
        self.with_attribute(http_equiv)
    }

    /// Value of the element.
    fn content(self, content: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Content(content.into()))
    }

    /// Character encoding declaration.
    fn charset(self, charset: Charset) -> Self {
        self.with_attribute(charset)
    }

    /// Applicable media.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Media(media.into()))
    }
}

/// Metadata name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);

/// The referrer information send when following a hyperlink.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum HttpEquiv {
    /// This pragma sets the pragma-set default language.
    ContentLanguage,

    /// Alternative form of setting the charset attribute.
    ContentType,

    /// Sets the name of the default CSS style sheet set.
    DefaultStyle,

    /// Acts as a timed redirect.
    Refresh,

    /// In practice, this pragma encourages Internet Explorer to more closely follow the
    /// specifications.
    XUaCompatible,

    /// Enforces a Content Security Policy on a Document.
    ContentSecurityPolicy,
}

impl fmt::Display for HttpEquiv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ContentLanguage => "content-language",
            Self::ContentType => "content-type",
            Self::DefaultStyle => "default-style",
            Self::Refresh => "refresh",
            Self::XUaCompatible => "x-ua-compatible",
            Self::ContentSecurityPolicy => "content-security-policy",
        })
    }
}

/// Value of the element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Content(pub Cow<'static, str>);

/// Character encoding declaration.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Charset {
    #[default]
    Utf8,
}

impl fmt::Display for Charset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Utf8 => "utf-8",
        })
    }
}

/// Applicable media.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Media(pub Cow<'static, str>);
