use std::borrow::Cow;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::link::Blocking;
use super::meta::Media;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `style` element allows authors to embed CSS style sheets in their documents. The `style`
/// element is one of several inputs to the styling processing model. The element does not
/// represent content for the user.
pub fn style() -> Element<marker::Style> {
    Element::new("style")
}

pub mod marker {
    pub struct Style;

    impl<'v, S: Into<std::borrow::Cow<'v, str>>> crate::element::IntoChild<'v, Style> for S {
        fn into_child(self) -> impl crate::View + 'v {
            self.into()
        }
    }
}

impl<E, P> Style<(marker::Style, P)> for E where E: ElementProxy<marker::Style, P> {}
impl<E, P> Common<(marker::Style, P)> for E where E: ElementProxy<marker::Style, P> {}
impl<E, P> Global<(marker::Style, P)> for E where E: ElementProxy<marker::Style, P> {}
impl<E, P> Aria<(marker::Style, P)> for E where E: ElementProxy<marker::Style, P> {}

/// The `style` element allows authors to embed CSS style sheets in their documents. The `style`
/// element is one of several inputs to the styling processing model. The element does not represent
/// content for the user.
pub trait Style<T>: WithAttribute {
    /// Applicable media.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Media(media.into()))
    }

    /// Indicate that the element is potentially render blocking.
    fn blocking(self) -> Self {
        self.with_blocking(true)
    }

    /// Indicate that the element is potentially render blocking.
    fn with_blocking(self, blocking: bool) -> Self {
        self.with_attribute(Blocking(blocking))
    }
}
