use std::borrow::Cow;

use cabin_macros::Attribute;

use super::anchor::ReferrerPolicy;
use super::common::Common;
use super::global::Global;
use super::link::{Blocking, CrossOrigin, FetchPriority};
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

// TODO: take str and not View
pub fn script(content: impl View) -> Html<marker::Script, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("script", (), content)
}

pub mod marker {
    pub struct Script;
}

impl<A: Attributes, V: 'static> Script for Html<marker::Script, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Script, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Script, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Script, A, V> {}

/// A `script` element allows to include dynamic script and data blocks in their documents.
pub trait Script: WithAttribute {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Src> {
        self.with_attribute(Src(src.into()))
    }

    /// The type of the script.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> Self::Output<Type> {
        self.with_attribute(Type(r#type.into()))
    }

    /// Whether to prevent execution in user agents that support module scripts.
    fn no_module(self) -> Self::Output<NoModule> {
        self.with_no_module(true)
    }

    /// Whether to prevent execution in user agents that support module scripts.
    fn with_no_module(self, no_module: bool) -> Self::Output<NoModule> {
        self.with_attribute(NoModule(no_module))
    }

    /// Execute script when available, without blocking while fetching.
    fn r#async(self) -> Self::Output<Async> {
        self.with_async(true)
    }

    /// Execute script when available, without blocking while fetching.
    fn with_async(self, r#async: bool) -> Self::Output<Async> {
        self.with_attribute(Async(r#async))
    }

    /// Defer script execution.
    fn defer(self) -> Self::Output<Defer> {
        self.with_defer(true)
    }

    /// Defer script execution.
    fn with_defer(self, defer: bool) -> Self::Output<Defer> {
        self.with_attribute(Defer(defer))
    }

    /// Handling of crossorigin requests.
    fn cross_origin(self, cross_origin: CrossOrigin) -> Self::Output<CrossOrigin> {
        self.with_attribute(cross_origin)
    }

    /// Integrity metadata used in _Subresource Integrity_ checks.
    fn integrity(self, integrity: impl Into<Cow<'static, str>>) -> Self::Output<Integrity> {
        self.with_attribute(Integrity(integrity.into()))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> Self::Output<ReferrerPolicy> {
        self.with_attribute(referrer_policy)
    }

    fn blocking(self) -> Self::Output<Blocking> {
        self.with_blocking(true)
    }

    fn with_blocking(self, blocking: bool) -> Self::Output<Blocking> {
        self.with_attribute(Blocking(blocking))
    }

    /// Sets the priority for fetches initiated by the element.
    fn fetch_priority(self, fetch_priority: FetchPriority) -> Self::Output<FetchPriority> {
        self.with_attribute(fetch_priority)
    }
}
/// Address of the resource.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Src(pub Cow<'static, str>);

/// The type of the script.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Type(pub Cow<'static, str>);

/// Whether to prevent execution in user agents that support module scripts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct NoModule(pub bool);

/// Execute script when available, without blocking while fetching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Async(pub bool);

/// Defer script execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Defer(pub bool);

/// Integrity metadata used in _Subresource Integrity_ checks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Integrity(pub Cow<'static, str>);
