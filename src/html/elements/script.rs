use std::borrow::Cow;

use cabin_macros::Element;

use super::anchor::ReferrerPolicy;
use super::link::{CrossOrigin, FetchPriority, RenderBlocking};

/// A `script` element allows to include dynamic script and data blocks in their documents.
#[derive(Default, Element)]
pub struct Script<Ext = ()> {
    pub extension: Ext,

    /// Address of the resource.
    pub src: Option<Cow<'static, str>>,

    /// The type of the script.
    pub r#type: Option<Cow<'static, str>>,

    /// Whether to prevent execution in user agents that support module scripts.
    #[element(attribute_name = "nomodule")]
    pub no_module: bool,

    /// Execute script when available, without blocking while fetching.
    #[element(method_name = "with_async")]
    pub r#async: bool,

    /// Defer script execution.
    #[element(method_name = "with_defer")]
    pub defer: bool,

    /// Handling of crossorigin requests.
    #[element(attribute_name = "crossorigin")]
    pub cross_origin: Option<CrossOrigin>,

    /// Integrity metadata used in _Subresource Integrity_ checks.
    pub integrity: Option<Cow<'static, str>>,

    /// How much referrer information to send.
    #[element(attribute_name = "referrerpolicy")]
    pub referrer_policy: ReferrerPolicy,

    #[element(skip)]
    pub blocking: Option<RenderBlocking>,

    /// Sets the priority for fetches initiated by the element.
    #[element(attribute_name = "fetchpriority")]
    pub fetch_priority: FetchPriority,
}

impl<V, Ext> ScriptElement<V, Ext> {
    /// Execute script when available, without blocking while fetching.
    pub fn r#async(mut self) -> Self {
        self.kind.r#async = true;
        self
    }

    /// Defer script execution.
    pub fn defer(mut self) -> Self {
        self.kind.defer = true;
        self
    }

    /// Indicate that the element is potentially render blocking.
    pub fn blocking(mut self) -> Self {
        self.kind.blocking = Some(RenderBlocking);
        self
    }
}
