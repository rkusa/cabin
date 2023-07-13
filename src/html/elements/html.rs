use cabin_macros::element;

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// The `html` element represents the root of an HTML document.
#[element]
pub trait Html: Common + Global + Aria {}
