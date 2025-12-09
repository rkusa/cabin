use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
pub fn dialog() -> Element<marker::Dialog> {
    Element::new("dialog")
}

pub mod marker {
    pub struct Dialog;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Dialog> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Dialog<(marker::Dialog, P)> for E where E: ElementProxy<marker::Dialog, P> {}
impl<E, P> Common<(marker::Dialog, P)> for E where E: ElementProxy<marker::Dialog, P> {}
impl<E, P> Global<(marker::Dialog, P)> for E where E: ElementProxy<marker::Dialog, P> {}
impl<E, P> Aria<(marker::Dialog, P)> for E where E: ElementProxy<marker::Dialog, P> {}

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
pub trait Dialog<T>: WithAttribute {
    /// Whether the dialog is showing.
    fn open(self) -> Self {
        self.with_open(true)
    }

    /// Whether the dialog is showing.
    fn with_open(self, open: bool) -> Self {
        self.with_attribute(Open(open))
    }
}

/// Whether the dialog is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Open(pub bool);
