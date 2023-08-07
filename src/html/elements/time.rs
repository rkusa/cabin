use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

/// The `time` element represents a datetime, in machine-readable form as the `datetime` attribute,
/// and in human-readable form as its content.
pub fn time(content: impl View) -> Html<marker::Time, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("time", (), content)
}

pub mod marker {
    pub struct Time;
}

impl<A: Attributes, V: 'static> Time for Html<marker::Time, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Time, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Time, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Time, A, V> {}

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
