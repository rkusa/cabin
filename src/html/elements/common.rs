use std::borrow::Cow;

use cabin_macros::Attributes;

use super::SerializeEventFn;

#[derive(Default, Attributes)]
pub struct CommonAttributes {
    /// Unique identifier across the document.
    pub id: Option<Cow<'static, str>>,

    /// The various classes that the element belongs to.
    pub class: Option<Cow<'static, str>>,

    pub on_click: Option<Box<SerializeEventFn>>,
}
