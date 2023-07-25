use std::borrow::Cow;

use cabin_macros::{element, Attribute};

use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::html::Aria;

// TODO:
#[element(tag = "option")]
pub trait SelectOption: Common + Global + Aria {
    /// Value of the option
    fn value(self, value: impl Into<Cow<'static, str>>) -> impl SelectOption {
        self.with(Value(value.into()))
    }

    /// Whether the control is checked.
    fn selected(self) -> impl SelectOption {
        self.with_selected(true)
    }

    /// Whether the control is selected.
    fn with_selected(self, selected: bool) -> impl SelectOption {
        self.with(Selected(selected))
    }
}

/// Whether the option is selected.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Selected(pub bool);
