use std::borrow::Cow;

use super::aria::Aria;
use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use super::time::Datetime;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `ins` element represents an addition to the document.
    pub fn ins(&self) -> Element<marker::Ins> {
        Element::new(self.acquire_renderer(), "ins")
    }
}

pub mod marker {
    pub struct Ins;
}

impl Ins for Element<marker::Ins> {}
impl Common for Element<marker::Ins> {}
impl Global for Element<marker::Ins> {}
impl Aria for Element<marker::Ins> {}

/// The `ins` element represents an addition to the document.
pub trait Ins: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }

    /// Machine-readable datetime/date/time of the change.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Datetime(datetime.into()))
    }
}
