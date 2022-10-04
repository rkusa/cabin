use std::fmt;

pub use crabweb_macros::component;
pub use render::{Renderer, ViewHashTree};
pub use view::{IntoView, View};

pub mod component;
pub mod html;
mod render;
mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

pub async fn render<M>(view: impl View<M>) -> Result<String, fmt::Error> {
    let mut r = Renderer::new();
    r = view.render(r).await?;
    Ok(r.end().view)
}
