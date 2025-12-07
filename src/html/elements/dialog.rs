use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
pub fn dialog() -> Element<marker::Dialog> {
    Element::new("dialog")
}

pub mod marker {
    pub struct Dialog;
}

impl Dialog for Element<marker::Dialog> {}
impl Common for Element<marker::Dialog> {}
impl Global for Element<marker::Dialog> {}
impl Aria for Element<marker::Dialog> {}

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
pub trait Dialog: WithAttribute {
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
