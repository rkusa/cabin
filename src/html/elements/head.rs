use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::view::UpdateView;

/// The `head` element represents a collection of metadata for the document.
pub fn head(content: impl View) -> UpdateView<Html<marker::Head, (), impl View>> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    UpdateView::hidden_on_update(Html::new("head", (), content))
}

pub mod marker {
    pub struct Head;
}

impl<A: Attributes, V: 'static> Global for UpdateView<Html<marker::Head, A, V>> {}
