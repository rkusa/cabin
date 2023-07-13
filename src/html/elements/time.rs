use std::borrow::Cow;

use cabin_macros::{element, Attribute};

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// The `time` element represents a datetime, in machine-readable form as the `datetime` attribute,
/// and in human-readable form as its content.
#[element]
pub trait Time: Common + Global + Aria {
    /// Machine-readable datetime/date/time of the element's contents.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> impl Time {
        self.with(Datetime(datetime.into()))
    }
}

/// Machine-readable datetime/date/time of the element's contents.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Datetime(pub Cow<'static, str>);
