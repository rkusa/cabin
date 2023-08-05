use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

// TODO
pub fn li(content: impl View) -> Html<marker::Li, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("li", (), content)
}

pub mod marker {
    pub struct Li;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Li, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Li, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Li, A, V> {}
