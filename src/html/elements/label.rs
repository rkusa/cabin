use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
#[crate::view_macro(crate::html::elements::label)]
pub fn label(content: impl View) -> Html<marker::Label, ()> {
    Html::new("label", (), content)
}

pub mod marker {
    pub struct Label;
}

impl<A: Attributes> Label for Html<marker::Label, A> {}
impl<A: Attributes> Common for Html<marker::Label, A> {}
impl<A: Attributes> Global for Html<marker::Label, A> {}
impl<A: Attributes> Aria for Html<marker::Label, A> {}

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
pub trait Label: WithAttribute {
    /// The id of the form control the label is the caption for.
    fn r#for(self, r#for: impl Into<Cow<'static, str>>) -> Self::Output<For> {
        self.with_attribute(For(r#for.into()))
    }
}

/// The id of the form control the label is the caption for.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct For(pub Cow<'static, str>);
