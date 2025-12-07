use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::anchor::ReferrerPolicy;
use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use super::script::Src;
use crate::attribute::WithAttribute;
use crate::element::Element;
use crate::html::list::SpaceSeparated;

/// The `iframe` element represents its content navigable.
pub fn iframe() -> Element<marker::IFrame> {
    Element::new("iframe")
}

pub mod marker {
    pub struct IFrame;
}

impl IFrame for Element<marker::IFrame> {}
impl Common for Element<marker::IFrame> {}
impl Global for Element<marker::IFrame> {}
impl Aria for Element<marker::IFrame> {}

/// The `iframe` element represents its content navigable.
pub trait IFrame: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Src(src.into()))
    }

    /// A document to render in the iframe.
    fn src_doc(self, src_src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(SrcDoc(src_src.into()))
    }

    /// Name of content navigable.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }

    /// Security rules for nested content.
    fn sandbox(self, rel: impl Into<SpaceSeparated<Sandbox>>) -> Self {
        self.with_attribute(SandboxList(rel.into()))
    }

    /// Permissions policy to be applied to the iframe's contents.
    fn allow(self, allow: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Allow(allow.into()))
    }

    /// Whether to allow the iframe's contents to use `requestFullscreen()`.
    fn allow_fullscreen(self) -> Self {
        self.with_allow_fullscreen(true)
    }

    /// Whether to allow the iframe's contents to use `requestFullscreen()`.
    fn with_allow_fullscreen(self, allow_fullscreen: bool) -> Self {
        self.with_attribute(AllowFullscreen(allow_fullscreen))
    }

    /// Vertical dimension.
    fn height(self, height: u32) -> Self {
        self.with_attribute(Height(height))
    }

    /// Horizontal dimension.
    fn width(self, width: u32) -> Self {
        self.with_attribute(Width(width))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self {
        self.with_attribute(referrer_policy)
    }

    /// Used when determining loading deferral.
    fn loading(self, loading: Loading) -> Self {
        self.with_attribute(loading)
    }
}

/// A document to render in the iframe.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct SrcDoc(pub Cow<'static, str>);

/// Name of content navigable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);

/// Relationship between the location in the document containing the hyperlink and the
/// destination resource.
#[derive(Debug, Clone, Hash, Attribute)]
#[attribute(name = "rel")]
pub struct SandboxList(pub SpaceSeparated<Sandbox>);

/// Relationship between the document and the linked resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sandbox {
    Downloads,
    Forms,
    Modals,
    OrientationLock,
    PointerLock,
    Popups,
    PopupsToEscapeSandbox,
    Presentation,
    SameOrigin,
    Scripts,
    TopNavigation,
    TopNavigationByUserActivation,
    TopNavigationToCustomProtocols,
}

impl fmt::Display for Sandbox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Sandbox::Downloads => "allow-downloads",
            Sandbox::Forms => "allow-forms",
            Sandbox::Modals => "allow-modals",
            Sandbox::OrientationLock => "allow-orientation-lock",
            Sandbox::PointerLock => "allow-pointer-lock",
            Sandbox::Popups => "allow-popups",
            Sandbox::PopupsToEscapeSandbox => "allow-popups-to-escape-sandbox",
            Sandbox::Presentation => "allow-presentation",
            Sandbox::SameOrigin => "allow-same-origin",
            Sandbox::Scripts => "allow-scripts",
            Sandbox::TopNavigation => "allow-top-navigation",
            Sandbox::TopNavigationByUserActivation => "allow-top-navigation-by-user-activation",
            Sandbox::TopNavigationToCustomProtocols => "allow-top-navigation-to-custom-protocols",
        })
    }
}

/// Permissions policy to be applied to the iframe's contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Allow(pub Cow<'static, str>);

/// Whether to allow the iframe's contents to use `requestFullscreen()`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct AllowFullscreen(pub bool);

/// The referrer information send when following a hyperlink.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Loading {
    /// Used to defer fetching a resource until some conditions are met.
    Lazy,

    /// Used to fetch a resource immediately; the default state.
    #[default]
    Eager,
}

impl fmt::Display for Loading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Lazy => "lazy",
            Self::Eager => "eager",
        })
    }
}
