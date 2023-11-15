use super::col::Span;
use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::Html;
use crate::View;

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub fn colgroup(content: impl View) -> Html<marker::Colgroup, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("colgroup", (), content)
}

pub mod marker {
    pub struct Colgroup;
}

impl<A: Attributes, V: 'static> Colgroup for Html<marker::Colgroup, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Colgroup, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Colgroup, A, V> {}

/// The `colgroup` element represents a group of one or more columns in the [super::table] that is
/// its parent, if it has a parent and that is a [super::table] element.
pub trait Colgroup: WithAttribute {
    /// Number of colgroupumns spanned by the element
    fn span(self, width: u32) -> Self::Output<Span> {
        self.with_attribute(Span(width))
    }
}
