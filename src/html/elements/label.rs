use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
pub fn label(content: impl View) -> Html<marker::Label, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("label", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! label {
        ($($x:tt)*) => {
            $crate::html::elements::label::label($crate::view![$($x)*])
        }
    }

    pub use label;
}

pub use macros::label;

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
