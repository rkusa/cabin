use cabin_macros::element;

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// The `body` element represents the body of an HTML document.
#[element]
pub trait Body: Common + Global + Aria {}
