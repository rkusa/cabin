use std::borrow::Cow;

use super::anchor::{Href, Target};
use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::void_element::VoidElement;

/// The `base` element allows authors to specify the document base URL for the purposes of
/// parsing URLs, and the name of the default navigable for the purposes of following
/// hyperlinks. The element does not represent any content beyond this information.
pub fn base() -> VoidElement<marker::Base> {
    VoidElement::new("base")
}

pub mod marker {
    pub struct Base;
}

impl Base for VoidElement<marker::Base> {}
impl Common for VoidElement<marker::Base> {}
impl Global for VoidElement<marker::Base> {}
impl Aria for VoidElement<marker::Base> {}

/// The `base` element allows authors to specify the document base URL for the purposes of parsing
/// URLs, and the name of the default navigable for the purposes of following hyperlinks. The
/// element does not represent any content beyond this information.
pub trait Base: WithAttribute {
    /// Address of the hyperlink.
    fn href(self, href: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Href(href.into()))
    }

    /// The _browsing context_ the link should be opened in.
    fn target(self, target: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Target(target.into()))
    }

    /// Try to open the link in a new tab.
    fn target_blank(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_blank")))
    }

    /// Open the link in the parent browsing context.
    fn target_parent(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_parent")))
    }

    /// Open the link in the topmost browsing context.
    fn target_top(self) -> Self {
        self.with_attribute(Target(Cow::Borrowed("_top")))
    }
}
