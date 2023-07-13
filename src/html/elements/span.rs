use cabin_macros::element;

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

// TODO: doc comment
#[element]
pub trait Span: Common + Global + Aria {}
