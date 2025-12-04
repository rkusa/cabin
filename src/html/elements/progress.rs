use std::borrow::Cow;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use super::meter::{Max, Value};
use crate::attribute::WithAttribute;
use crate::context::Context;
use crate::element::Element;

impl Context {
    /// The `progress` element represents the completion progress of a task. The progress is either
    /// indeterminate, indicating that progress is being made but that it is not clear how much more
    /// work remains to be done before the task is complete (e.g. because the task is waiting for a
    /// remote host to respond), or the progress is a number in the range zero to a maximum, giving
    /// the fraction of work that has so far been completed.
    pub fn progress(&self) -> Element<'_, marker::Progress> {
        Element::new(self, "progress")
    }
}

pub mod marker {
    pub struct Progress;
}

impl<'v> Progress for Element<'v, marker::Progress> {}
impl<'v> Common for Element<'v, marker::Progress> {}
impl<'v> Global for Element<'v, marker::Progress> {}
impl<'v> Aria for Element<'v, marker::Progress> {}

/// The `progress` element represents the completion progress of a task. The progress is either
/// indeterminate, indicating that progress is being made but that it is not clear how much more
/// work remains to be done before the task is complete (e.g. because the task is waiting for a
/// remote host to respond), or the progress is a number in the range zero to a maximum, giving the
/// fraction of work that has so far been completed.
pub trait Progress: WithAttribute {
    /// Current value of the element.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Value(value.into()))
    }

    /// Upper bound of range.
    fn max(self, max: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Max(max.into()))
    }
}
