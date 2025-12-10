use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `meta` element represents various kinds of metadata that cannot be expressed using the
/// [super::title::title], [super::base::base], [super::link::link], [super::style::style], and
/// [super::script::script] elements.
pub fn meta(content: impl View) -> Html<marker::Meta, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("meta", (), content)
}

pub mod marker {
    pub struct Meta;
}

impl<A: Attributes> Meta for Html<marker::Meta, A> {}
impl<A: Attributes> Common for Html<marker::Meta, A> {}
impl<A: Attributes> Global for Html<marker::Meta, A> {}
impl<A: Attributes> Aria for Html<marker::Meta, A> {}

/// The `meta` element represents various kinds of metadata that cannot be expressed using the
/// [super::title::title], [super::base::base], [super::link::link], [super::style::style], and
/// [super::script::script] elements.
pub trait Meta: WithAttribute {
    /// Metadata name.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }

    /// Pragma directive.
    fn http_equiv(self, http_equiv: HttpEquiv) -> Self::Output<HttpEquiv> {
        self.with_attribute(http_equiv)
    }

    /// Value of the element.
    fn content(self, content: impl Into<Cow<'static, str>>) -> Self::Output<Content> {
        self.with_attribute(Content(content.into()))
    }

    /// Character encoding declaration.
    fn charset(self, charset: Charset) -> Self::Output<Charset> {
        self.with_attribute(charset)
    }

    /// Applicable media.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self::Output<Media> {
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
