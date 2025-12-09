use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
pub fn label() -> Element<marker::Label> {
    Element::new("label")
}

pub mod marker {
    pub struct Label;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Label> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<P> Label<marker::Label> for P where P: ElementProxy<marker::Label> {}
impl<P> Common<marker::Label> for P where P: ElementProxy<marker::Label> {}
impl<P> Global<marker::Label> for P where P: ElementProxy<marker::Label> {}
impl<P> Aria<marker::Label> for P where P: ElementProxy<marker::Label> {}

/// A `label` element that represents a caption that can be associated with a specific form
/// control.
pub trait Label<T>: WithAttribute {
    /// The id of the form control the label is the caption for.
    fn r#for(self, r#for: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(For(r#for.into()))
    }
}

/// The id of the form control the label is the caption for.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct For(pub Cow<'static, str>);
