use std::borrow::Cow;
use std::hash::Hash;

use cabin_macros::Attribute;

use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `meter` element represents a scalar measurement within a known range, or a fractional value;
/// for example disk usage, the relevance of a query result, or the fraction of a voting population
/// to have selected a particular candidate.
pub fn meter(content: impl View) -> Html<marker::Meter, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("meter", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! meter {
        ($($x:tt)*) => {
            $crate::html::elements::meter::meter($crate::view![$($x)*])
        }
    }

    pub use meter;
}

pub use macros::meter;

pub mod marker {
    pub struct Meter;
}

impl<A: Attributes> Meter for Html<marker::Meter, A> {}
impl<A: Attributes> Common for Html<marker::Meter, A> {}
impl<A: Attributes> Global for Html<marker::Meter, A> {}
impl<A: Attributes> Aria for Html<marker::Meter, A> {}

/// The `meter` element represents a scalar measurement within a known range, or a fractional value;
/// for example disk usage, the relevance of a query result, or the fraction of a voting population
/// to have selected a particular candidate.
pub trait Meter: WithAttribute {
    /// Current value of the element.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }

    /// Lower bound of range.
    fn min(self, min: impl Into<Cow<'static, str>>) -> Self::Output<Min> {
        self.with_attribute(Min(min.into()))
    }

    /// Upper bound of range.
    fn max(self, max: impl Into<Cow<'static, str>>) -> Self::Output<Max> {
        self.with_attribute(Max(max.into()))
    }

    /// High limit of low range.
    fn low(self, low: impl Into<Cow<'static, str>>) -> Self::Output<Low> {
        self.with_attribute(Low(low.into()))
    }

    /// Low limit of high range.
    fn high(self, high: impl Into<Cow<'static, str>>) -> Self::Output<High> {
        self.with_attribute(High(high.into()))
    }

    /// Optimum value in gauge.
    fn optimum(self, optimum: impl Into<Cow<'static, str>>) -> Self::Output<Optimum> {
        self.with_attribute(Optimum(optimum.into()))
    }
}

/// Current value of the element (represented as a string to avoid float precision issues).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Value(pub Cow<'static, str>);

/// Lower bound of range (represented as a string to avoid float precision issues).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Min(pub Cow<'static, str>);

/// Upper bound of range (represented as a string to avoid float precision issues).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Max(pub Cow<'static, str>);

/// High limit of low range (represented as a string to avoid float precision issues).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Low(pub Cow<'static, str>);

/// Low limit of high range (represented as a string to avoid float precision issues).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct High(pub Cow<'static, str>);

/// Optimum value in gauge (represented as a string to avoid float precision issues).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Optimum(pub Cow<'static, str>);
