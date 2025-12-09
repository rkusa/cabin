use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The `map` element, in conjunction with an [super::img] element and any [super::area] element
/// descendants, defines an image map. The element represents its children.
pub fn map() -> Element<marker::Map> {
    Element::new("map")
}

pub mod marker {
    pub struct Map;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Map> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Map<(marker::Map, P)> for E where E: ElementProxy<marker::Map, P> {}
impl<E, P> Common<(marker::Map, P)> for E where E: ElementProxy<marker::Map, P> {}
impl<E, P> Global<(marker::Map, P)> for E where E: ElementProxy<marker::Map, P> {}
impl<E, P> Aria<(marker::Map, P)> for E where E: ElementProxy<marker::Map, P> {}

/// The `map` element, in conjunction with an [super::img] element and any [super::area] element
/// descendants, defines an image map. The element represents its children.
pub trait Map<T>: WithAttribute {
    /// Name of image map to reference from the [super::img::Img::use_map] attribute.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }
}

/// Name of image map to reference from the [super::img::Img::use_map] attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);
