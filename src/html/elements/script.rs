use std::borrow::Cow;

use cabin_macros::Element;

use super::anchor::ReferrerPolicy;
use super::link::{CrossOrigin, FetchPriority, RenderBlocking};

/// A `script` element allows to include dynamic script and data blocks in their documents.
#[derive(Default, Element)]
pub struct Script {
    /// Address of the resource.
    src: Option<Cow<'static, str>>,

    /// The type of the script.
    #[element(attribute_name = "type")]
    r#type: Option<Cow<'static, str>>,

    /// Whether to prevent execution in user agents that support module scripts.
    #[element(attribute_name = "nomodule")]
    no_module: bool,

    /// Execute script when available, without blocking while fetching.
    #[element(method_name = "with_async")]
    #[element(attribute_name = "async")]
    r#async: bool,

    /// Defer script execution.
    #[element(method_name = "with_defer")]
    defer: bool,

    /// Handling of crossorigin requests.
    #[element(attribute_name = "crossorigin")]
    cross_origin: Option<CrossOrigin>,

    /// Integrity metadata used in _Subresource Integrity_ checks.
    integrity: Option<Cow<'static, str>>,

    /// How much referrer information to send.
    #[element(attribute_name = "referrerpolicy")]
    referrer_policy: ReferrerPolicy,

    #[element(skip)]
    blocking: Option<RenderBlocking>,

    /// Sets the priority for fetches initiated by the element.
    #[element(attribute_name = "fetchpriority")]
    fetch_priority: FetchPriority,
}

impl<Ext> ScriptElement<Ext> {
    /// Execute script when available, without blocking while fetching.
    pub fn r#async(mut self) -> Self {
        self.base.r#async = true;
        self
    }

    /// Defer script execution.
    pub fn defer(mut self) -> Self {
        self.base.defer = true;
        self
    }

    /// Indicate that the element is potentially render blocking.
    pub fn blocking(mut self) -> Self {
        self.base.blocking = Some(RenderBlocking);
        self
    }
}
