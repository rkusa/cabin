use std::fmt;

use cabin_macros::Attribute;

use super::aria::Aria;
use super::common::Common;
use super::global::Global;
use crate::attribute::WithAttribute;
use crate::element::{Element, ElementProxy};

/// The ol element represents a list of items, where the items have been intentionally ordered,
/// such that changing the order would change the meaning of the document.
pub fn ol() -> Element<marker::Ol> {
    Element::new("ol")
}

pub mod marker {
    pub struct Ol;

    impl<'v, V: crate::View + 'v> crate::element::IntoChild<'v, Ol> for V {
        fn into_child(self) -> impl crate::View {
            self
        }
    }
}

impl<E, P> Ol<(marker::Ol, P)> for E where E: ElementProxy<marker::Ol, P> {}
impl<E, P> Common<(marker::Ol, P)> for E where E: ElementProxy<marker::Ol, P> {}
impl<E, P> Global<(marker::Ol, P)> for E where E: ElementProxy<marker::Ol, P> {}
impl<E, P> Aria<(marker::Ol, P)> for E where E: ElementProxy<marker::Ol, P> {}

/// The ol element represents a list of items, where the items have been intentionally ordered, such
/// that changing the order would change the meaning of the document.
pub trait Ol<T>: WithAttribute {
    /// Number the list backwards.
    fn reversed(self, reversed: bool) -> Self {
        self.with_attribute(Reversed(reversed))
    }

    /// Starting value of the list.
    fn start(self, start: u32) -> Self {
        self.with_attribute(Start(start))
    }

    /// Kind of list marker.
    fn r#type(self, r#type: Type) -> Self {
        self.with_attribute(r#type)
    }

    fn type_decimal(self) -> Self {
        self.with_attribute(Type::Decimal)
    }

    fn type_lower_alpha(self) -> Self {
        self.with_attribute(Type::LowerAlpha)
    }

    fn type_upper_alpha(self) -> Self {
        self.with_attribute(Type::UpperAlpha)
    }

    fn type_lower_roman(self) -> Self {
        self.with_attribute(Type::LowerRoman)
    }

    fn type_upper_roman(self) -> Self {
        self.with_attribute(Type::UpperRoman)
    }
}

/// Number the list backwards.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Reversed(pub bool);

/// Starting value of the list.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Start(pub u32);

/// Data type of an input element.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub enum Type {
    /// Decimal numbers, e.g. 1., 2., 3., ...
    Decimal,

    /// Lowercase latin alphabet, e.g. a., b., h::, ...
    LowerAlpha,

    /// Uppercase latin alphabet, e.g. A., B., C., ...
    UpperAlpha,

    /// Lowercase roman numerals, e.g. i., ii., iii., ...
    LowerRoman,

    /// Uppercase roman numerals, e.g. I., II., III., ...
    UpperRoman,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Decimal => "1",
            Self::LowerAlpha => "a",
            Self::UpperAlpha => "A",
            Self::LowerRoman => "i",
            Self::UpperRoman => "I",
        })
    }
}
