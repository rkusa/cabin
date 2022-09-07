use std::fmt;

use action::Action;
pub use component::Component;
pub use crabweb_macros::{action, component, event};
use view::HashTree;
pub use view::{Render, View};

pub mod action;
pub mod component;
pub mod html;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

pub fn render(view: impl View<()>) -> Result<String, fmt::Error> {
    let mut hash_tree = HashTree::default();
    let renderer = view.render(&mut hash_tree);
    let mut result = String::new();
    renderer.render(&mut result)?;
    Ok(result)
}
