use std::borrow::Cow;

use cabin_macros::Element;

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
#[derive(Default, Element)]
pub struct Label {
    /// The id of the form control the label is the caption for.
    #[element(attribute_name = "for")]
    r#for: Option<Cow<'static, str>>,
}
