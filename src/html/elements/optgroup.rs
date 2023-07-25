use std::borrow::Cow;

use cabin_macros::{element, Attribute};

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

// TODO:
#[element]
pub trait OptGroup: Common + Global + Aria {
    fn label(self, value: impl Into<Cow<'static, str>>) -> impl OptGroup {
        self.with(Label(value.into()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Attribute)]
pub struct Label(pub Cow<'static, str>);
