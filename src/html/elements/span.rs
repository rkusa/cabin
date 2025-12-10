use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};

// TODO:
pub fn span(content: impl View) -> Html<marker::Span, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("span", (), content)
}

pub mod marker {
    pub struct Span;
}

impl<A: Attributes> Common for Html<marker::Span, A> {}
impl<A: Attributes> Global for Html<marker::Span, A> {}
impl<A: Attributes> Aria for Html<marker::Span, A> {}
