use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};

/// The `html` element represents the root of an HTML document.
#[crate::view_macro(crate::html::elements::html)]
pub fn html(content: impl View) -> Html<marker::Html, ()> {
    Html::new("html", (), content)
}

pub mod marker {
    pub struct Html;
}

impl<A: Attributes> Global for Html<marker::Html, A> {}
