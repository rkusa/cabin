use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::element::{Element, ElementProxy};

// TODO:
pub fn span() -> Element<marker::Span> {
    Element::new("span")
}

pub mod marker {
    pub struct Span;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Span> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Common<marker::Span> for P where P: ElementProxy<marker::Span> {}
impl<P> Global<marker::Span> for P where P: ElementProxy<marker::Span> {}
impl<P> Aria<marker::Span> for P where P: ElementProxy<marker::Span> {}
