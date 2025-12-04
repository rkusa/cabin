use std::borrow::Cow;

use super::anchor::{Href, Target};
use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::void_element::VoidElement;

impl Context {
    /// The `base` element allows authors to specify the document base URL for the purposes of
    /// parsing URLs, and the name of the default navigable for the purposes of following
    /// hyperlinks. The element does not represent any content beyond this information.
    pub fn base(&self) -> VoidElement<'_, marker::Base> {
        VoidElement::new(self, "base")
    }
}

pub mod marker {
    pub struct Base;
}

impl<'v> Base for VoidElement<'_, marker::Base> {}
impl<'v> Common for VoidElement<'_, marker::Base> {}
impl<'v> Global for VoidElement<'_, marker::Base> {}
impl<'v> Aria for VoidElement<'_, marker::Base> {}

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
