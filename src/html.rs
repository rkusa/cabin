use crate::context::Context;
use crate::html::raw::Raw;

pub mod elements;
pub mod events;
pub mod list;
pub mod raw;

impl Context {
    pub fn doctype(&self) -> Raw<'static> {
        raw::raw("<!DOCTYPE html>")
    }
}
