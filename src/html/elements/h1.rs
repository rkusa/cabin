use cabin_macros::element;

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// TODO:
#[element]
pub trait H1: Common + Global + Aria {}
