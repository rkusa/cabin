#![forbid(unsafe_code)]
#![feature(async_fn_in_trait, return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]
#![allow(unused)]

use std::future::Future;

use bytes::Bytes;
pub use cabin_macros::{action, component, signal};
pub use error::Error;
use http::{HeaderValue, Response};
use render::Renderer;
use scope::Scope;
// pub use render::{Renderer, ViewHashTree};
// pub use restore::Restored;
pub use view::View;

// pub mod component;
pub mod actions;
pub mod error;
pub mod html;
mod local_pool;
pub mod private;
pub mod render;
mod scope;
// mod restore;
pub mod signal;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

// TODO: move behind feature flag?
pub fn cabin_stylesheets() -> impl View {
    r#"<link rel="stylesheet" href="/styles.css">"#
}

pub fn cabin_scripts() -> impl View {
    r#"<script src="/server-component.js" async></script>"#
}

pub async fn render_to_response<F: Future<Output = V>, V: View + 'static>(
    render_fn: impl FnOnce() -> F + Send + Sync + 'static,
) -> Response<Bytes> {
    let (scope, result) = local_pool::spawn(move || async move {
        let scope = Scope::new();
        let result = scope
            .clone()
            .run(async move {
                let r = Renderer::new();
                render_fn().await.render(r).await
            })
            .await;
        (scope.into_view(), result)
    })
    .await;
    let result = match result {
        Ok(result) => result,
        Err(err) => {
            eprintln!(
                "{err}\n{}",
                format_caused_by(std::error::Error::source(&err))
            );
            return err.into();
        }
    };

    let html = result.end().view;
    Response::builder()
        .header(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        )
        .body(Bytes::from(format!(
            "{html}\n\
            <script type=\"application/json\" id=\"state\">{scope}</script>"
        )))
        .unwrap()
}

fn dispatch(action: &str, state: Bytes) {
    //
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
