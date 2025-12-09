use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `time` element represents a datetime, in machine-readable form as the `datetime`
/// attribute, and in human-readable form as its content.
pub fn time() -> Element<marker::Time> {
    Element::new("time")
}

pub mod marker {
    pub struct Time;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Time> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Time<marker::Time> for P where P: ElementProxy<marker::Time> {}
impl<P> Common<marker::Time> for P where P: ElementProxy<marker::Time> {}
impl<P> Global<marker::Time> for P where P: ElementProxy<marker::Time> {}
impl<P> Aria<marker::Time> for P where P: ElementProxy<marker::Time> {}

/// The `time` element represents a datetime, in machine-readable form as the `datetime` attribute,
/// and in human-readable form as its content.
pub trait Time<T>: WithAttribute {
    /// Machine-readable datetime/date/time of the element's contents.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Datetime(datetime.into()))
    }
}

/// Machine-readable datetime/date/time of the element's contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Datetime(pub Cow<'static, str>);
