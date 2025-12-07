use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `blockquote` element represents a section that is quoted from another source.
    /// Content inside a `blockquote` must be quoted from another source, whose address, if it has
    /// one, may be cited in the [Blockquote::cite] attribute.
    pub fn blockquote(&self) -> Element<marker::Blockquote> {
        Element::new(self.acquire_renderer(), "blockquote")
    }
}

pub mod marker {
    pub struct Blockquote;
}

impl Blockquote for Element<marker::Blockquote> {}
impl Common for Element<marker::Blockquote> {}
impl Global for Element<marker::Blockquote> {}
impl Aria for Element<marker::Blockquote> {}

/// The `blockquote` element represents a section that is quoted from another source.
/// Content inside a `blockquote` must be quoted from another source, whose address, if it has one,
/// may be cited in the [Blockquote::cite] attribute.
pub trait Blockquote: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }
}

/// Link to the source of the quotation or more information about the edit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Cite(pub Cow<'static, str>);
