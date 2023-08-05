use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

pub fn optgroup(content: impl View) -> Html<marker::OptGroup, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("optgroup", (), content)
}

pub mod marker {
    pub struct OptGroup;
}

impl<A: Attributes, V: 'static> OptGroup for Html<marker::OptGroup, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::OptGroup, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::OptGroup, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::OptGroup, A, V> {}

// TODO:
pub trait OptGroup: WithAttribute {
    fn label(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Label> {
        self.with_attribute(Label(value.into()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Label(pub Cow<'static, str>);
