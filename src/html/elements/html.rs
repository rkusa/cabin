use super::global::Global;
use crate::element::{Element, ElementProxy};

/// The `html` element represents the root of an HTML document.
pub fn html() -> Element<marker::Html> {
    Element::new("html")
}

pub mod marker {
    pub struct Html;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Html> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Global<marker::Html> for P where P: ElementProxy<marker::Html> {}
