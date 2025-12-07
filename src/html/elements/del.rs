use std::borrow::Cow;

use super::aria::Aria;
use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use super::time::Datetime;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// The `del` element represents a removal from the document.
pub fn del() -> Element<marker::Del> {
    Element::new("del")
}

pub mod marker {
    pub struct Del;
}

impl Del for Element<marker::Del> {}
impl Common for Element<marker::Del> {}
impl Global for Element<marker::Del> {}
impl Aria for Element<marker::Del> {}

/// The `del` element represents a removal from the document.
pub trait Del: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }

    /// Machine-readable datetime/date/time of the change.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Datetime(datetime.into()))
    }
}
