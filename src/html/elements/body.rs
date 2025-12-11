use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Common, Global, Html};

/// The `body` element represents the body of an HTML document.
#[crate::view_macro(crate::html::elements::body)]
pub fn body(content: impl View) -> Html<marker::Body, ()> {
    Html::new("body", (), content)
}

pub mod marker {
    pub struct Body;
}

impl<A: Attributes> Common for Html<marker::Body, A> {}
impl<A: Attributes> Global for Html<marker::Body, A> {}
