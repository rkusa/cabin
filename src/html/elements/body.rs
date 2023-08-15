use crate::html::attributes::Attributes;
use crate::html::{Common, Global, Html};
use crate::view::UpdateView;
use crate::View;

/// The `body` element represents the body of an HTML document.
pub fn body(content: impl View) -> UpdateView<Html<marker::Body, (), impl View>> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    UpdateView::content_only_on_update(Html::new("body", (), content))
}

pub mod marker {
    pub struct Body;
}

impl<A: Attributes, V: 'static> Common for UpdateView<Html<marker::Body, A, V>> {}
impl<A: Attributes, V: 'static> Global for UpdateView<Html<marker::Body, A, V>> {}
