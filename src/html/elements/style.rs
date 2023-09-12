use std::borrow::Cow;

use super::common::Common;
use super::global::Global;
use super::link::Blocking;
use super::meta::Media;
use super::script::ScriptEscape;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

/// The `style` element allows authors to embed CSS style sheets in their documents. The `style`
/// element is one of several inputs to the styling processing model. The element does not represent
/// content for the user.
pub fn style(content: impl Into<Cow<'static, str>>) -> Html<marker::Style, (), impl View> {
    Html::new("style", (), ScriptEscape(content.into()))
}

pub mod marker {
    pub struct Style;
}

impl<A: Attributes, V: 'static> Style for Html<marker::Style, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Style, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Style, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Style, A, V> {}

/// The `style` element allows authors to embed CSS style sheets in their documents. The `style`
/// element is one of several inputs to the styling processing model. The element does not represent
/// content for the user.
pub trait Style: WithAttribute {
    /// Applicable media.
    fn media(self, media: impl Into<Cow<'static, str>>) -> Self::Output<Media> {
        self.with_attribute(Media(media.into()))
    }

    /// Indicate that the element is potentially render blocking.
    fn blocking(self) -> Self::Output<Blocking> {
        self.with_blocking(true)
    }

    /// Indicate that the element is potentially render blocking.
    fn with_blocking(self, blocking: bool) -> Self::Output<Blocking> {
        self.with_attribute(Blocking(blocking))
    }
}
