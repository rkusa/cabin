use std::borrow::Cow;

use super::aria::Aria;
use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// The `q` element represents some phrasing content quoted from another source.
pub fn q() -> Element<marker::Q> {
    Element::new("q")
}

pub mod marker {
    pub struct Q;
}

impl Q for Element<marker::Q> {}
impl Common for Element<marker::Q> {}
impl Global for Element<marker::Q> {}
impl Aria for Element<marker::Q> {}

/// The `q` element represents some phrasing content quoted from another source.
pub trait Q: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }
}
