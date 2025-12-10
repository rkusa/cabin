use super::common::Common;
use super::dialog::Open;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `details` element represents a disclosure widget from which the user can obtain additional
/// information or controls.
pub fn details(content: impl View) -> Html<marker::Details, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("details", (), content)
}

pub mod marker {
    pub struct Details;
}

impl<A: Attributes> Details for Html<marker::Details, A> {}
impl<A: Attributes> Common for Html<marker::Details, A> {}
impl<A: Attributes> Global for Html<marker::Details, A> {}
impl<A: Attributes> Aria for Html<marker::Details, A> {}

/// The `details` element represents a disclosure widget from which the user can obtain additional
/// information or controls.
pub trait Details: WithAttribute {
    /// Whether the details is visible.
    fn open(self) -> Self::Output<Open> {
        self.with_open(true)
    }

    /// Whether the details is visible.
    fn with_open(self, open: bool) -> Self::Output<Open> {
        self.with_attribute(Open(open))
    }
}
