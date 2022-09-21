#![allow(unused)]

use std::fmt;

use action::Action;
pub use component::Component;
pub use crabweb_macros::{action, component, event};
use render::Renderer;
pub use render::ViewHashTree;
pub use view::{IntoView, Text, View};

pub mod action;
pub mod component;
pub mod html;
mod render;
mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

pub fn render<S>(view: impl View<S>) -> Result<String, fmt::Error> {
    let mut renderer = Renderer::new();
    view.render(&mut renderer)?;
    Ok(renderer.end().view)
}
