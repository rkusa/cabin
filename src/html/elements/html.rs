use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

/// The `html` element represents the root of an HTML document.
pub fn html(content: impl View) -> Html<marker::Html, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("html", (), content)
}

pub mod marker {
    pub struct Html;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Html, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Html, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Html, A, V> {}
