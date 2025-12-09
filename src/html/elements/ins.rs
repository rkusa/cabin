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

impl<P> Ins<marker::Ins> for P where P: ElementProxy<marker::Ins> {}
impl<P> Common<marker::Ins> for P where P: ElementProxy<marker::Ins> {}
impl<P> Global<marker::Ins> for P where P: ElementProxy<marker::Ins> {}
impl<P> Aria<marker::Ins> for P where P: ElementProxy<marker::Ins> {}

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
