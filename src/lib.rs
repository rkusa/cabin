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

pub async fn render(view: impl View) -> Result<String, fmt::Error> {
    let mut r = Renderer::new();
    r = view.render(r).await?;
    let html = r.end().view;
    const STYLESHEET: &str = r#"<link rel="stylesheet" href="/styles.css">"#;
    const SCRIPT: &str = r#"<script src="/server-component.js" async></script>"#;
    let html = format!("{STYLESHEET}\n{SCRIPT}\n{html}");
    Ok(html)
}
