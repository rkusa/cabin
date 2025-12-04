use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `time` element represents a datetime, in machine-readable form as the `datetime`
    /// attribute, and in human-readable form as its content.
    pub fn time(&self) -> Element<'_, marker::Time> {
        Element::new(self, "time")
    }
}

pub mod marker {
    pub struct Time;
}

impl<'v> Time for Element<'v, marker::Time> {}
impl<'v> Common for Element<'v, marker::Time> {}
impl<'v> Global for Element<'v, marker::Time> {}
impl<'v> Aria for Element<'v, marker::Time> {}

/// The `time` element represents a datetime, in machine-readable form as the `datetime` attribute,
/// and in human-readable form as its content.
pub trait Time: WithAttribute {
    /// Machine-readable datetime/date/time of the element's contents.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Datetime(datetime.into()))
    }
}

/// Machine-readable datetime/date/time of the element's contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Datetime(pub Cow<'static, str>);
