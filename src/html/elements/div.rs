use cabin_macros::element;

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// The `div` element represents a generic container for flow content.
#[element]
pub trait Div: Common + Global + Aria {}
