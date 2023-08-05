use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

pub fn dialog(content: impl View) -> Html<marker::Dialog, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("dialog", (), content)
}

pub mod marker {
    pub struct Dialog;
}

impl<A: Attributes, V: 'static> Dialog for Html<marker::Dialog, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Dialog, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Dialog, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Dialog, A, V> {}

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
pub trait Dialog: WithAttribute {
    /// Whether the dialog is showing.
    fn open(self) -> Self::Output<Open> {
        self.with_open(true)
    }

    /// Whether the dialog is showing.
    fn with_open(self, open: bool) -> Self::Output<Open> {
        self.with_attribute(Open(open))
    }
}

/// Whether the dialog is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Open(pub bool);
