use std::fmt;

use action::Action;
pub use component::Component;
pub use crabweb_macros::{action, component, event};
use view::HashTree;
pub use view::{Render, Text, View};

pub mod action;
pub mod component;
pub mod html;
mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

pub fn render<S>(view: impl View<S>) -> Result<String, fmt::Error> {
    let mut hash_tree = HashTree::default();
    let renderer = view.prepare(&mut hash_tree);
    let mut result = String::new();
    renderer.render(&mut result, false)?;
    Ok(result)
}
