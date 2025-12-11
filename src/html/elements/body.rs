use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Common, Global, Html};
use crate::view::UpdateView;

/// The `body` element represents the body of an HTML document.
#[crate::view_macro(cabin::html::elements::body)]
pub fn body(content: impl View) -> UpdateView<Html<marker::Body, ()>> {
    UpdateView::content_only_on_update(Html::new("body", (), content))
}

pub mod marker {
    pub struct Body;
}

impl<A: Attributes> Common for UpdateView<Html<marker::Body, A>> {}
impl<A: Attributes> Global for UpdateView<Html<marker::Body, A>> {}
