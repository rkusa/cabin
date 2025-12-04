use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// A `dialog` element represents a transitory part of an application (e.g. dialog box).
    pub fn dialog(&self) -> Element<'_, marker::Dialog> {
        Element::new(self, "dialog")
    }
}

pub mod marker {
    pub struct Dialog;
}

impl<'v> Dialog for Element<'v, marker::Dialog> {}
impl<'v> Common for Element<'v, marker::Dialog> {}
impl<'v> Global for Element<'v, marker::Dialog> {}
impl<'v> Aria for Element<'v, marker::Dialog> {}

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
