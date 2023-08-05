use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

/// The `body` element represents the body of an HTML document.
pub fn body(content: impl View) -> Html<marker::Body, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("body", (), content)
}

pub mod marker {
    pub struct Body;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Body, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Body, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Body, A, V> {}
