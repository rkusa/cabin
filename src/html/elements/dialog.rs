use cabin_macros::{element, Attribute};

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
#[element]
pub trait Dialog: Common + Global + Aria {
    /// Whether the dialog is showing.
    fn open(self) -> impl Dialog {
        self.with_open(true)
    }

    /// Whether the dialog is showing.
    fn with_open(self, open: bool) -> impl Dialog {
        self.with(Open(open))
    }
}

/// Whether the dialog is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Open(pub bool);
