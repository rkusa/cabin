use super::col::Span;
use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::Html;
use crate::html::attributes::{Attributes, WithAttribute};

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub fn colgroup(content: impl View) -> Html<marker::Colgroup, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("colgroup", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! colgroup {
        ($($x:tt)*) => {
            $crate::html::elements::colgroup::colgroup($crate::view![$($x)*])
        }
    }

    pub use colgroup;
}

pub use macros::colgroup;

pub mod marker {
    pub struct Colgroup;
}

impl<A: Attributes> Colgroup for Html<marker::Colgroup, A> {}
impl<A: Attributes> Common for Html<marker::Colgroup, A> {}
impl<A: Attributes> Global for Html<marker::Colgroup, A> {}

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub trait Colgroup: WithAttribute {
    /// Number of colgroupumns spanned by the element
    fn span(self, width: u32) -> Self::Output<Span> {
        self.with_attribute(Span(width))
    }
}
