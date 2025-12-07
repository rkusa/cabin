use super::aria::Aria;
use super::common::Common;
use super::dialog::Open;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// The `details` element represents a disclosure widget from which the user can obtain
/// additional information or controls.
pub fn details() -> Element<marker::Details> {
    Element::new("details")
}

pub mod marker {
    pub struct Details;
}

impl Details for Element<marker::Details> {}
impl Common for Element<marker::Details> {}
impl Global for Element<marker::Details> {}
impl Aria for Element<marker::Details> {}

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
