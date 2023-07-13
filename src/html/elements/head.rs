use cabin_macros::element;

use super::common::Common;
use super::global::Global;
use crate::html::Aria;

/// The `head` element represents a collection of metadata for a document.
#[element]
pub trait Head: Common + Global + Aria {}
