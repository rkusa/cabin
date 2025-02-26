use std::borrow::Cow;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `map` element, in conjunction with an [super::img] element and any [super::area] element
/// descendants, defines an image map. The element represents its children.
pub fn map(content: impl View) -> Html<marker::Map, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("map", (), content)
}

pub mod marker {
    pub struct Map;
}

impl<A: Attributes, V: 'static> Map for Html<marker::Map, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Map, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Map, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Map, A, V> {}

/// The `map` element, in conjunction with an [super::img] element and any [super::area] element
/// descendants, defines an image map. The element represents its children.
pub trait Map: WithAttribute {
    /// Name of image map to reference from the [super::img::Img::use_map] attribute.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }
}

/// Name of image map to reference from the [super::img::Img::use_map] attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);
