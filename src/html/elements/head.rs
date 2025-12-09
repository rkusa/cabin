use super::global::Global;
use crate::element::{Element, ElementProxy};

/// The `head` element represents a collection of metadata for the document.
pub fn head() -> Element<marker::Head> {
    Element::new("head")
}

pub mod marker {
    pub struct Head;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Head> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Global<marker::Head> for P where P: ElementProxy<marker::Head> {}
