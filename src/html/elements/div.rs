use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

/// The `div` element represents a generic container for flow content.
pub fn div(content: impl View) -> Html<marker::Div, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("div", (), content)
}

pub mod marker {
    pub struct Div;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Div, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Div, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Div, A, V> {}
