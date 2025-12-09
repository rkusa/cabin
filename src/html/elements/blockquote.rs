use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `blockquote` element represents a section that is quoted from another source.
/// Content inside a `blockquote` must be quoted from another source, whose address, if it has
/// one, may be cited in the [Blockquote::cite] attribute.
pub fn blockquote() -> Element<marker::Blockquote> {
    Element::new("blockquote")
}

pub mod marker {
    pub struct Blockquote;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Blockquote> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Blockquote<marker::Blockquote> for P where P: ElementProxy<marker::Blockquote> {}
impl<P> Common<marker::Blockquote> for P where P: ElementProxy<marker::Blockquote> {}
impl<P> Global<marker::Blockquote> for P where P: ElementProxy<marker::Blockquote> {}
impl<P> Aria<marker::Blockquote> for P where P: ElementProxy<marker::Blockquote> {}

/// The `blockquote` element represents a section that is quoted from another source.
/// Content inside a `blockquote` must be quoted from another source, whose address, if it has one,
/// may be cited in the [Blockquote::cite] attribute.
pub trait Blockquote<T>: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Cite(src.into()))
    }
}

/// Link to the source of the quotation or more information about the edit.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Cite(pub Cow<'static, str>);
