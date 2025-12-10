use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `blockquote` element represents a section that is quoted from another source.
/// Content inside a `blockquote` must be quoted from another source, whose address, if it has one,
/// may be cited in the [Blockquote::cite] attribute.
pub fn blockquote(content: impl View) -> Html<marker::Blockquote, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("blockquote", (), content)
}

pub mod marker {
    pub struct Blockquote;
}

impl<A: Attributes> Blockquote for Html<marker::Blockquote, A> {}
impl<A: Attributes> Common for Html<marker::Blockquote, A> {}
impl<A: Attributes> Global for Html<marker::Blockquote, A> {}
impl<A: Attributes> Aria for Html<marker::Blockquote, A> {}

/// The `blockquote` element represents a section that is quoted from another source.
/// Content inside a `blockquote` must be quoted from another source, whose address, if it has one,
/// may be cited in the [Blockquote::cite] attribute.
pub trait Blockquote: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Cite> {
        self.with_attribute(Cite(src.into()))
    }
}

/// Link to the source of the quotation or more information about the edit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Cite(pub Cow<'static, str>);
