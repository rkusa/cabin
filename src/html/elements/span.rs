use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

// TODO:
pub fn span(content: impl View) -> Html<marker::Span, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("span", (), content)
}

pub mod marker {
    pub struct Span;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Span, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Span, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Span, A, V> {}
