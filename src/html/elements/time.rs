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

impl<E, P> Time<(marker::Time, P)> for E where E: ElementProxy<marker::Time, P> {}
impl<E, P> Common<(marker::Time, P)> for E where E: ElementProxy<marker::Time, P> {}
impl<E, P> Global<(marker::Time, P)> for E where E: ElementProxy<marker::Time, P> {}
impl<E, P> Aria<(marker::Time, P)> for E where E: ElementProxy<marker::Time, P> {}

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
