use std::borrow::Cow;

use cabin_macros::Element;

/// The `time` element represents a datetime, in machine-readable form as the `datetime` attribute,
/// and in human-readable form as its content.
#[derive(Default, Element)]
pub struct TimeAttributes {
    /// Machine-readable datetime/date/time of the element's contents.
    datetime: Option<Cow<'static, str>>,
}
