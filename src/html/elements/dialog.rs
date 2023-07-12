use cabin_macros::Element;
use cabin_macros::{Attributes2, Element};

use crate::html::attributes::{Attributes2, Pair};

/// A `dialog` element represents a transitory part of an application (e.g. dialog box).
#[derive(Default, Element)]
pub struct DialogAttributes {
    /// Whether the dialog is showing.
    open: bool,
}
