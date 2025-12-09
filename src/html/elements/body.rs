use super::common::Common;
use super::global::Global;
use crate::element::{Element, ElementProxy};

/// The `body` element represents the body of an HTML document.
pub fn body() -> Element<marker::Body> {
    Element::new("body")
}

pub mod marker {
    pub struct Body;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Body> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Common<marker::Body> for P where P: ElementProxy<marker::Body> {}
impl<P> Global<marker::Body> for P where P: ElementProxy<marker::Body> {}
