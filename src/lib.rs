#![forbid(unsafe_code)]

use std::fmt;

pub use render::{Renderer, ViewHashTree};
pub use rustend_macros::component;
pub use view::{IntoView, View};

pub mod component;
pub mod html;
pub mod previous;
pub mod private;
mod render;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

// TODO: move behind feature flag?
pub fn rustend_stylesheets() -> impl View {
    r#"<link rel="stylesheet" href="/styles.css">"#
}

pub fn rustend_scripts() -> impl View {
    r#"<script src="/server-component.js" async></script>"#
}

pub async fn render(view: impl View) -> Result<String, fmt::Error> {
    let mut r = Renderer::new();
    r = view.render(r).await?;
    let html = r.end().view;
    Ok(html)
}
