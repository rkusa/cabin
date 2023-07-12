use std::borrow::Cow;

use cabin_macros::Element;

use super::anchor::ReferrerPolicy;
use super::link::{CrossOrigin, FetchPriority, RenderBlocking};
use crate::html::attributes::Attributes;

/// A `script` element allows to include dynamic script and data blocks in their documents.
#[derive(Default, Element)]
pub struct ScriptAttributes {
    /// Address of the resource.
    src: Option<Cow<'static, str>>,

    /// The type of the script.
    #[attributes(attribute_name = "type")]
    r#type: Option<Cow<'static, str>>,

    /// Whether to prevent execution in user agents that support module scripts.
    #[attributes(attribute_name = "nomodule")]
    no_module: bool,

    /// Execute script when available, without blocking while fetching.
    #[attributes(method_name = "with_async")]
    #[attributes(attribute_name = "async")]
    r#async: bool,

    /// Defer script execution.
    #[attributes(method_name = "with_defer")]
    defer: bool,

    /// Handling of crossorigin requests.
    #[attributes(attribute_name = "crossorigin")]
    cross_origin: Option<CrossOrigin>,

    /// Integrity metadata used in _Subresource Integrity_ checks.
    integrity: Option<Cow<'static, str>>,

    /// How much referrer information to send.
    #[attributes(attribute_name = "referrerpolicy")]
    referrer_policy: ReferrerPolicy,

    #[attributes(skip)]
    blocking: Option<RenderBlocking>,

    /// Sets the priority for fetches initiated by the element.
    #[attributes(attribute_name = "fetchpriority")]
    fetch_priority: FetchPriority,
}

pub trait ScriptExt: AsMut<ScriptAttributes> + Sized {
    /// Execute script when available, without blocking while fetching.
    fn r#async(mut self) -> Self {
        self.as_mut().r#async = true;
        self
    }

    /// Defer script execution.
    fn defer(mut self) -> Self {
        self.as_mut().defer = true;
        self
    }

    /// Indicate that the element is potentially render blocking.
    fn blocking(mut self) -> Self {
        self.as_mut().blocking = Some(RenderBlocking);
        self
    }
}

impl ScriptExt for Attributes<ScriptAttributes> {}
