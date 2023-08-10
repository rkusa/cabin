use std::borrow::Cow;

use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::Html;

/// The `data` element represents its contents, along with a machine-readable form of those contents
/// in the [Data::value] attribute.
pub fn data() -> Html<marker::Data, (), ()> {
    Html::new("data", (), ())
}

pub mod marker {
    pub struct Data;
}

impl<A: Attributes, V: 'static> Data for Html<marker::Data, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Data, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Data, A, V> {}

/// The `data` element represents a group of one or more columns in the [Table] that is its
/// parent, if it has a parent and that is a [Table] element.
pub trait Data: WithAttribute {
    /// Machine-readable value.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }
}
