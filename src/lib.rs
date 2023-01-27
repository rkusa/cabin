#![forbid(unsafe_code)]

use std::fmt;

pub use render::{Renderer, ViewHashTree};
pub use rustend_macros::{component, css};
pub use view::{IntoView, View};

pub mod component;
pub mod html;
pub mod previous;
mod render;
pub mod style;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

pub async fn render(view: impl View) -> Result<String, fmt::Error> {
    let mut r = Renderer::new();
    r = view.render(r).await?;
    Ok(r.end().view)
}
