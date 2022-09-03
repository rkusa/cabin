use std::fmt;

pub use action::Action;
pub use component::Component;
pub use view::View;

mod action;
mod component;
pub mod html;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

pub fn render(view: impl View<()>) -> Result<String, fmt::Error> {
    let mut result = String::new();
    view.render(&mut result)?;
    Ok(result)
}
