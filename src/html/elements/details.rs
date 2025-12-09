use super::aria::Aria;
use super::common::Common;
use super::dialog::Open;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `details` element represents a disclosure widget from which the user can obtain
/// additional information or controls.
pub fn details() -> Element<marker::Details> {
    Element::new("details")
}

pub mod marker {
    pub struct Details;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Details> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Details<(marker::Details, P)> for E where E: ElementProxy<marker::Details, P> {}
impl<E, P> Common<(marker::Details, P)> for E where E: ElementProxy<marker::Details, P> {}
impl<E, P> Global<(marker::Details, P)> for E where E: ElementProxy<marker::Details, P> {}
impl<E, P> Aria<(marker::Details, P)> for E where E: ElementProxy<marker::Details, P> {}

/// The `details` element represents a disclosure widget from which the user can obtain additional
/// information or controls.
pub trait Details<T>: WithAttribute {
    /// Whether the details is visible.
    fn open(self) -> Self {
        self.with_open(true)
    }

    /// Whether the details is visible.
    fn with_open(self, open: bool) -> Self {
        self.with_attribute(Open(open))
    }
}
