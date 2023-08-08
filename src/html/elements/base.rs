use std::borrow::Cow;

use super::anchor::{Href, Target};
use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `base` element allows authors to specify the document base URL for the purposes of parsing
/// URLs, and the name of the default navigable for the purposes of following hyperlinks. The
/// element does not represent any content beyond this information.
pub fn base() -> Html<marker::Base, (), ()> {
    Html::new("base", (), ())
}

pub mod marker {
    pub struct Base;
}

impl<A: Attributes, V: 'static> Base for Html<marker::Base, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Base, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Base, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Base, A, V> {}

/// The `base` element allows authors to specify the document base URL for the purposes of parsing
/// URLs, and the name of the default navigable for the purposes of following hyperlinks. The
/// element does not represent any content beyond this information.
pub trait Base: WithAttribute {
    /// Address of the hyperlink.
    fn href(self, href: impl Into<Cow<'static, str>>) -> Self::Output<Href> {
        self.with_attribute(Href(href.into()))
    }

    /// The _browsing context_ the link should be opened in.
    fn target(self, target: impl Into<Cow<'static, str>>) -> Self::Output<Target> {
        self.with_attribute(Target(target.into()))
    }

    /// Try to open the link in a new tab.
    fn target_blank(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_blank")))
    }

    /// Open the link in the parent browsing context.
    fn target_parent(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_parent")))
    }

    /// Open the link in the topmost browsing context.
    fn target_top(self) -> Self::Output<Target> {
        self.with_attribute(Target(Cow::Borrowed("_top")))
    }
}
