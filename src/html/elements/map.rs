use std::borrow::Cow;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::Element;

/// The `map` element, in conjunction with an [super::img] element and any [super::area] element
/// descendants, defines an image map. The element represents its children.
pub fn map() -> Element<marker::Map> {
    Element::new("map")
}

pub mod marker {
    pub struct Map;
}

impl Map for Element<marker::Map> {}
impl Common for Element<marker::Map> {}
impl Global for Element<marker::Map> {}
impl Aria for Element<marker::Map> {}

/// The `map` element, in conjunction with an [super::img] element and any [super::area] element
/// descendants, defines an image map. The element represents its children.
pub trait Map: WithAttribute {
    /// Name of image map to reference from the [super::img::Img::use_map] attribute.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self {
        self.with_attribute(Name(name.into()))
    }
}

/// Name of image map to reference from the [super::img::Img::use_map] attribute.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Name(pub Cow<'static, str>);
