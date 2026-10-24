use std::borrow::Cow;
use std::fmt;

use cabin_macros::Attribute;

use super::anchor::ReferrerPolicy;
use super::common::Common;
use super::global::Global;
use super::input::{Height, Width};
use super::script::Src;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::list::SpaceSeparated;
use crate::html::{Aria, Html};

/// The `iframe` element represents its content navigable.
#[crate::view_macro(crate::html::elements::iframe)]
pub fn iframe(content: impl View) -> Html<marker::IFrame, ()> {
    Html::new("iframe", (), content)
}

pub mod marker {
    pub struct IFrame;
}

impl<A: Attributes> IFrame for Html<marker::IFrame, A> {}
impl<A: Attributes> Common for Html<marker::IFrame, A> {}
impl<A: Attributes> Global for Html<marker::IFrame, A> {}
impl<A: Attributes> Aria for Html<marker::IFrame, A> {}

/// The `iframe` element represents its content navigable.
pub trait IFrame: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Src> {
        self.with_attribute(Src(src.into()))
    }

    /// A document to render in the iframe.
    fn src_doc(self, src_src: impl Into<Cow<'static, str>>) -> Self::Output<SrcDoc> {
        self.with_attribute(SrcDoc(src_src.into()))
    }

    /// Name of content navigable.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }

    /// Security rules for nested content.
    fn sandbox(self, sandbox: impl Into<SpaceSeparated<Sandbox>>) -> Self::Output<SandboxList> {
        self.with_attribute(SandboxList(sandbox.into()))
    }

    /// Appends a [Sandbox] to the security rules.
    fn append_sandbox(mut self, sandbox: Sandbox) -> Self::Output<SandboxList> {
        let sandbox_list = if let Some(list) = self.get_attribute_mut::<SandboxList>() {
            SandboxList(
                match std::mem::replace(&mut list.0, SpaceSeparated::Single(Sandbox::Downloads)) {
                    SpaceSeparated::Single(existing) => {
                        SpaceSeparated::List([existing, sandbox].into())
                    }
                    SpaceSeparated::List(mut list) => {
                        list.insert(sandbox);
                        SpaceSeparated::List(list)
                    }
                },
            )
        } else {
            SandboxList(SpaceSeparated::Single(sandbox))
        };
        self.with_attribute(sandbox_list)
    }

    /// Permissions policy to be applied to the iframe's contents.
    fn allow(self, allow: impl Into<Cow<'static, str>>) -> Self::Output<Allow> {
        self.with_attribute(Allow(allow.into()))
    }

    /// Whether to allow the iframe's contents to use `requestFullscreen()`.
    fn allow_fullscreen(self) -> Self::Output<AllowFullscreen> {
        self.with_allow_fullscreen(true)
    }

    /// Whether to allow the iframe's contents to use `requestFullscreen()`.
    fn with_allow_fullscreen(self, allow_fullscreen: bool) -> Self::Output<AllowFullscreen> {
        self.with_attribute(AllowFullscreen(allow_fullscreen))
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

    /// Used when determining loading deferral.
    fn loading(self, loading: Loading) -> Self::Output<Loading> {
        self.with_attribute(loading)
    }
}

/// A document to render in the iframe.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct SrcDoc(pub Cow<'static, str>);

/// Name of content navigable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);

/// Security rules for nested content.
#[derive(Debug, Clone, Hash, Attribute)]
#[attribute(name = "sandbox")]
pub struct SandboxList(pub SpaceSeparated<Sandbox>);

/// A security rule for nested content, lifting one of the `sandbox` restrictions.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Renderer;

    #[tokio::test]
    async fn test_sandbox() {
        assert_eq!(
            iframe(())
                .sandbox(Sandbox::Scripts)
                .append_sandbox(Sandbox::Forms)
                .render(Renderer::new(false, true))
                .await
                .unwrap()
                .end()
                .html,
            r#"<iframe sandbox="allow-forms allow-scripts"></iframe>"#
        );
    }
}
