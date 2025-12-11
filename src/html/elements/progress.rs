use std::borrow::Cow;

use super::common::Common;
use super::global::Global;
use super::meter::{Max, Value};
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `progress` element represents the completion progress of a task. The progress is either
/// indeterminate, indicating that progress is being made but that it is not clear how much more
/// work remains to be done before the task is complete (e.g. because the task is waiting for a
/// remote host to respond), or the progress is a number in the range zero to a maximum, giving the
/// fraction of work that has so far been completed.
#[crate::view_macro(cabin::html::elements::progress)]
pub fn progress(content: impl View) -> Html<marker::Progress, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("progress", (), content)
}

pub mod marker {
    pub struct Progress;
}

impl<A: Attributes> Progress for Html<marker::Progress, A> {}
impl<A: Attributes> Common for Html<marker::Progress, A> {}
impl<A: Attributes> Global for Html<marker::Progress, A> {}
impl<A: Attributes> Aria for Html<marker::Progress, A> {}

/// The `progress` element represents the completion progress of a task. The progress is either
/// indeterminate, indicating that progress is being made but that it is not clear how much more
/// work remains to be done before the task is complete (e.g. because the task is waiting for a
/// remote host to respond), or the progress is a number in the range zero to a maximum, giving the
/// fraction of work that has so far been completed.
pub trait Progress: WithAttribute {
    /// Current value of the element.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }

    /// Upper bound of range.
    fn max(self, max: impl Into<Cow<'static, str>>) -> Self::Output<Max> {
        self.with_attribute(Max(max.into()))
    }
}
