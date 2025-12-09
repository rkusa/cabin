use std::borrow::Cow;

use super::aria::Aria;
use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use super::time::Datetime;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `del` element represents a removal from the document.
pub fn del() -> Element<marker::Del> {
    Element::new("del")
}

pub mod marker {
    pub struct Del;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Del> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Del<(marker::Del, P)> for E where E: ElementProxy<marker::Del, P> {}
impl<E, P> Common<(marker::Del, P)> for E where E: ElementProxy<marker::Del, P> {}
impl<E, P> Global<(marker::Del, P)> for E where E: ElementProxy<marker::Del, P> {}
impl<E, P> Aria<(marker::Del, P)> for E where E: ElementProxy<marker::Del, P> {}

/// The `del` element represents a removal from the document.
pub trait Del<T>: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }

    /// Machine-readable datetime/date/time of the change.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Datetime(datetime.into()))
    }
}
