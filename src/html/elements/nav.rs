use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

// TODO:
pub fn nav(content: impl View) -> Html<marker::Nav, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("nav", (), content)
}

pub mod marker {
    pub struct Nav;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Nav, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Nav, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Nav, A, V> {}
