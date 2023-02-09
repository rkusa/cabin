#![forbid(unsafe_code)]

use bytes::Bytes;
pub use error::Error;
use http::{HeaderValue, Response};
pub use render::{Renderer, ViewHashTree};
pub use rustend_macros::component;
pub use view::{IntoView, View};

pub mod component;
pub mod error;
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

pub async fn render_to_response(view: impl View) -> Response<Bytes> {
    let r = Renderer::new();
    match view.render(r).await.map(|r| r.end().view) {
        Ok(html) => Response::builder()
            .header(
                http::header::CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )
            .body(Bytes::from(html))
            .unwrap(),
        Err(err) => {
            eprintln!(
                "{err}\n{}",
                format_caused_by(std::error::Error::source(&err))
            );
            err.into()
        }
    }
}

fn format_caused_by(source: Option<&dyn std::error::Error>) -> String {
    use std::fmt::Write;

    let mut caused_by = String::new();

    let mut source = source;
    let mut i = 0;

    // if source.is_some() {
    //     caused_by += "\n\nCaused by:\n";
    // }

    while let Some(err) = source {
        if i > 0 {
            writeln!(&mut caused_by).ok();
        }
        write!(&mut caused_by, "{i:>4}: {err}").ok();
        source = err.source().or_else(|| {
            #[allow(deprecated)]
            err.cause()
        });
        i += 1;
    }

    caused_by
}
