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

impl<E, P> Common<(marker::Span, P)> for E where E: ElementProxy<marker::Span, P> {}
impl<E, P> Global<(marker::Span, P)> for E where E: ElementProxy<marker::Span, P> {}
impl<E, P> Aria<(marker::Span, P)> for E where E: ElementProxy<marker::Span, P> {}
