use super::common::Common;
use super::global::Global;
use crate::html::attributes::Attributes;
use crate::html::{Aria, Html};
use crate::View;

// TODO:
pub fn fieldset(content: impl View) -> Html<marker::Fieldset, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("fieldset", (), content)
}

pub mod marker {
    pub struct Fieldset;
}

impl<A: Attributes, V: 'static> Common for Html<marker::Fieldset, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Fieldset, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Fieldset, A, V> {}
