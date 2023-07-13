use std::borrow::Cow;

use cabin_macros::{element, Attribute};

use super::anchor::ReferrerPolicy;
use super::common::Common;
use super::global::Global;
use super::link::{Blocking, CrossOrigin, FetchPriority};
use crate::html::Aria;

/// A `script` element allows to include dynamic script and data blocks in their documents.
#[element]
pub trait Script: Common + Global + Aria {
    /// Address of the resource.
    fn src(self, src: impl Into<Cow<'static, str>>) -> impl Script {
        self.with(Src(src.into()))
    }

    /// The type of the script.
    fn r#type(self, r#type: impl Into<Cow<'static, str>>) -> impl Script {
        self.with(Type(r#type.into()))
    }

    /// Whether to prevent execution in user agents that support module scripts.
    fn no_module(self) -> impl Script {
        self.with_no_module(true)
    }

    /// Whether to prevent execution in user agents that support module scripts.
    fn with_no_module(self, no_module: bool) -> impl Script {
        self.with(NoModule(no_module))
    }

    /// Execute script when available, without blocking while fetching.
    fn r#async(self) -> impl Script {
        self.with_async(true)
    }

    /// Execute script when available, without blocking while fetching.
    fn with_async(self, r#async: bool) -> impl Script {
        self.with(Async(r#async))
    }

    /// Defer script execution.
    fn defer(self) -> impl Script {
        self.with_defer(true)
    }

    /// Defer script execution.
    fn with_defer(self, defer: bool) -> impl Script {
        self.with(Defer(defer))
    }

    /// Handling of crossorigin requests.
    fn cross_origin(self, cross_origin: CrossOrigin) -> impl Script {
        self.with(cross_origin)
    }

    /// Integrity metadata used in _Subresource Integrity_ checks.
    fn integrity(self, integrity: impl Into<Cow<'static, str>>) -> impl Script {
        self.with(Integrity(integrity.into()))
    }

    /// How much referrer information to send.
    fn referrer_policy(self, referrer_policy: ReferrerPolicy) -> impl Script {
        self.with(referrer_policy)
    }

    fn blocking(self) -> impl Script {
        self.with_blocking(true)
    }

    fn with_blocking(self, blocking: bool) -> impl Script {
        self.with(Blocking(blocking))
    }

    /// Sets the priority for fetches initiated by the element.
    fn fetch_priority(self, fetch_priority: FetchPriority) -> impl Script {
        self.with(fetch_priority)
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
