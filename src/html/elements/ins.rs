use std::borrow::Cow;

use super::aria::Aria;
use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use super::time::Datetime;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `ins` element represents an addition to the document.
pub fn ins() -> Element<marker::Ins> {
    Element::new("ins")
}

pub mod marker {
    pub struct Ins;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Ins> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Ins<(marker::Ins, P)> for E where E: ElementProxy<marker::Ins, P> {}
impl<E, P> Common<(marker::Ins, P)> for E where E: ElementProxy<marker::Ins, P> {}
impl<E, P> Global<(marker::Ins, P)> for E where E: ElementProxy<marker::Ins, P> {}
impl<E, P> Aria<(marker::Ins, P)> for E where E: ElementProxy<marker::Ins, P> {}

/// The `ins` element represents an addition to the document.
pub trait Ins<T>: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }

    /// Machine-readable datetime/date/time of the change.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Datetime(datetime.into()))
    }
}
