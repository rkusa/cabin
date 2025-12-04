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
    /// The `del` element represents a removal from the document.
    pub fn del(&self) -> Element<'_, marker::Del> {
        Element::new(self, "del")
    }
}

pub mod marker {
    pub struct Del;
}

impl<'v> Del for Element<'v, marker::Del> {}
impl<'v> Common for Element<'v, marker::Del> {}
impl<'v> Global for Element<'v, marker::Del> {}
impl<'v> Aria for Element<'v, marker::Del> {}

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
