use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

// TODO:
pub fn h1(content: impl View) -> Html<marker::H1, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("h1", (), content)
}

pub mod marker {
    pub struct H1;
}

impl<A: Attributes, V: 'static> Common for Html<marker::H1, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::H1, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::H1, A, V> {}
