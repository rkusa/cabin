use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::view::UpdateView;

/// The `html` element represents the root of an HTML document.
#[crate::view_macro(cabin::html::elements::html)]
pub fn html(content: impl View) -> UpdateView<Html<marker::Html, ()>> {
    UpdateView::content_only_on_update(Html::new("html", (), content))
}

pub mod marker {
    pub struct Html;
}

impl<A: Attributes> Global for UpdateView<Html<marker::Html, A>> {}
