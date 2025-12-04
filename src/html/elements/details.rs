use super::aria::Aria;
use super::common::Common;
use super::dialog::Open;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `details` element represents a disclosure widget from which the user can obtain
    /// additional information or controls.
    pub fn details(&self) -> Element<'_, marker::Details> {
        Element::new(self, "details")
    }
}

pub mod marker {
    pub struct Details;
}

impl<'v> Details for Element<'v, marker::Details> {}
impl<'v> Common for Element<'v, marker::Details> {}
impl<'v> Global for Element<'v, marker::Details> {}
impl<'v> Aria for Element<'v, marker::Details> {}

/// The `details` element represents a disclosure widget from which the user can obtain additional
/// information or controls.
pub trait Details: WithAttribute {
    /// Whether the details is visible.
    fn open(self) -> Self {
        self.with_open(true)
    }

    /// Whether the details is visible.
    fn with_open(self, open: bool) -> Self {
        self.with_attribute(Open(open))
    }
}
