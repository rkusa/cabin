use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

pub fn label(content: impl View) -> Html<marker::Label, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("label", (), content)
}

pub mod marker {
    pub struct Label;
}

impl<A: Attributes, V: 'static> Label for Html<marker::Label, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Label, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Label, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Label, A, V> {}

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
