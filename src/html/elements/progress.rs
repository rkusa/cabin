use std::borrow::Cow;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::meter::{Max, Value};
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `progress` element represents the completion progress of a task. The progress is either
/// indeterminate, indicating that progress is being made but that it is not clear how much more
/// work remains to be done before the task is complete (e.g. because the task is waiting for a
/// remote host to respond), or the progress is a number in the range zero to a maximum, giving
/// the fraction of work that has so far been completed.
pub fn progress() -> Element<marker::Progress> {
    Element::new("progress")
}

pub mod marker {
    pub struct Progress;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Progress> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Progress<marker::Progress> for P where P: ElementProxy<marker::Progress> {}
impl<P> Common<marker::Progress> for P where P: ElementProxy<marker::Progress> {}
impl<P> Global<marker::Progress> for P where P: ElementProxy<marker::Progress> {}
impl<P> Aria<marker::Progress> for P where P: ElementProxy<marker::Progress> {}

/// The `progress` element represents the completion progress of a task. The progress is either
/// indeterminate, indicating that progress is being made but that it is not clear how much more
/// work remains to be done before the task is complete (e.g. because the task is waiting for a
/// remote host to respond), or the progress is a number in the range zero to a maximum, giving the
/// fraction of work that has so far been completed.
pub trait Progress<T>: WithAttribute {
    /// Current value of the element.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Value(value.into()))
    }

    /// Upper bound of range.
    fn max(self, max: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Max(max.into()))
    }
}
