use std::borrow::Cow;

use cabin_macros::{element, Attribute};

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
#[element]
pub trait Label: Common + Global + Aria {
    /// The id of the form control the label is the caption for.
    fn r#for(self, r#for: impl Into<Cow<'static, str>>) -> impl Label {
        self.with(For(r#for.into()))
    }
}

/// The id of the form control the label is the caption for.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct For(pub Cow<'static, str>);
