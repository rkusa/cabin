use super::common::Common;
use super::dialog::Open;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

/// The `details` element represents a disclosure widget from which the user can obtain additional
/// information or controls.
pub fn details(content: impl View) -> Html<marker::Details, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("details", (), content)
}

pub mod marker {
    pub struct Details;
}

impl<A: Attributes, V: 'static> Details for Html<marker::Details, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Details, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Details, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Details, A, V> {}

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
