use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::view::UpdateView;

/// The `head` element represents a collection of metadata for the document.
#[crate::view_macro(crate::html::elements::head)]
pub fn head(content: impl View) -> UpdateView<Html<marker::Head, ()>> {
    UpdateView::hidden_on_update(Html::new("head", (), content))
}

pub mod marker {
    pub struct Head;
}

impl<A: Attributes> Global for UpdateView<Html<marker::Head, A>> {}
