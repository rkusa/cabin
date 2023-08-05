use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

/// The `head` element represents a collection of metadata for a document.
pub fn head(content: impl View) -> Html<marker::Head, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("head", (), content)
}

pub mod marker {
    pub struct Head;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Head, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Head, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Head, A, V> {}
