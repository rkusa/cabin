use std::borrow::Cow;

use super::aria::Aria;
use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `q` element represents some phrasing content quoted from another source.
    pub fn q(&self) -> Element<'_, marker::Q> {
        Element::new(self, "q")
    }
}

pub mod marker {
    pub struct Q;
}

impl<'v> Q for Element<'v, marker::Q> {}
impl<'v> Common for Element<'v, marker::Q> {}
impl<'v> Global for Element<'v, marker::Q> {}
impl<'v> Aria for Element<'v, marker::Q> {}

/// The `q` element represents some phrasing content quoted from another source.
pub trait Q: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }
}
