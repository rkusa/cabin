use cabin_macros::{Attributes2, Element};
use std::borrow::Cow;

use crate::html::attributes::{Attributes2, Pair};

use cabin_macros::Element;

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
#[derive(Default, Element)]
pub struct LabelAttributes {
    /// The id of the form control the label is the caption for.
    #[attributes(attribute_name = "for")]
    r#for: Option<Cow<'static, str>>,
}
