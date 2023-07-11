use std::borrow::Cow;

use cabin_macros::Element;

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
#[derive(Default, Element)]
pub struct Label<Ext = ()> {
    pub extension: Ext,

    /// The id of the form control the label is the caption for.
    pub r#for: Option<Cow<'static, str>>,
}
