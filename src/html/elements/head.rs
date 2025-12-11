use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};

/// The `head` element represents a collection of metadata for the document.
#[crate::view_macro(crate::html::elements::head)]
pub fn head(content: impl View) -> Html<marker::Head, ()> {
    Html::new("head", (), content)
}

pub mod marker {
    pub struct Head;
}

impl<A: Attributes> Global for Html<marker::Head, A> {}
