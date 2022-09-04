use std::fmt;

use action::Action;
pub use component::Component;
pub use crabweb_macros::{action, component, event};
pub use view::View;

pub mod action;
pub mod component;
pub mod html;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

pub fn render(view: impl View<()>) -> Result<String, fmt::Error> {
    let mut result = String::new();
    view.render(&mut result)?;
    Ok(result)
}
