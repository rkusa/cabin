use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

// TODO:
pub fn ul(content: impl View) -> Html<marker::Ul, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("ul", (), content)
}

pub mod marker {
    pub struct Ul;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Ul, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Ul, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Ul, A, V> {}
