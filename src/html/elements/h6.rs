use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

// TODO:
pub fn h6(content: impl View) -> Html<marker::H6, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("h6", (), content)
}

pub mod marker {
    pub struct H6;
}

impl<A: Attributes, V: 'static> Common for Html<marker::H6, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::H6, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::H6, A, V> {}
