use std::borrow::Cow;

use super::aria::Aria;
use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `q` element represents some phrasing content quoted from another source.
pub fn q() -> Element<marker::Q> {
    Element::new("q")
}

pub mod marker {
    pub struct Q;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Q> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Q<(marker::Q, P)> for E where E: ElementProxy<marker::Q, P> {}
impl<E, P> Common<(marker::Q, P)> for E where E: ElementProxy<marker::Q, P> {}
impl<E, P> Global<(marker::Q, P)> for E where E: ElementProxy<marker::Q, P> {}
impl<E, P> Aria<(marker::Q, P)> for E where E: ElementProxy<marker::Q, P> {}

/// The `q` element represents some phrasing content quoted from another source.
pub trait Q<T>: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }
}
