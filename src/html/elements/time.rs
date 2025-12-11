use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `time` element represents a datetime, in machine-readable form as the `datetime` attribute,
/// and in human-readable form as its content.
#[crate::view_macro(crate::html::elements::time)]
pub fn time(content: impl View) -> Html<marker::Time, ()> {
    Html::new("time", (), content)
}

pub mod marker {
    pub struct Time;
}

impl<A: Attributes> Time for Html<marker::Time, A> {}
impl<A: Attributes> Common for Html<marker::Time, A> {}
impl<A: Attributes> Global for Html<marker::Time, A> {}
impl<A: Attributes> Aria for Html<marker::Time, A> {}

/// The `time` element represents a datetime, in machine-readable form as the `datetime` attribute,
/// and in human-readable form as its content.
pub trait Time: WithAttribute {
    /// Machine-readable datetime/date/time of the element's contents.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self::Output<Datetime> {
        self.with_attribute(Datetime(datetime.into()))
    }
}

/// Machine-readable datetime/date/time of the element's contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Datetime(pub Cow<'static, str>);
