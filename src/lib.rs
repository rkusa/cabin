#![forbid(unsafe_code)]

extern crate self as cabin;

use std::collections::HashMap;
use std::future::Future;

use bytes::Bytes;
pub use error::Error;
pub use http::StatusCode;
use http::{HeaderValue, Response};
use http_body::Full;
use render::Renderer;
use scope::Scope;
use serde_json::value::RawValue;
use state::StateId;
pub use view::View;

pub mod error;
pub mod html;
mod local_pool;
pub mod private;
pub mod render;
pub mod scope;
pub mod state;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");

// TODO: move behind feature flag?
pub fn cabin_stylesheets() -> impl View {
    r#"<link rel="stylesheet" href="/styles.css">"#
}

pub fn cabin_scripts() -> impl View {
    r#"<script src="/server-component.js" async></script>"#
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    event_id: u32,
    state: HashMap<StateId, Box<RawValue>>,
    payload: Box<RawValue>,
}

pub async fn get_page<F: Future<Output = V>, V: View + 'static>(
    render_fn: impl FnOnce() -> F + Send + Sync + 'static,
) -> Response<Full<Bytes>> {
    let (scope, result) = local_pool::spawn(move || async move {
        let scope = Scope::new();
        let result = scope
            .clone()
            .run(async move {
                let r = Renderer::new();
                let body = render_fn().await;
                html::custom(
                    "html",
                    (
                        html::custom("head", (cabin_stylesheets(), cabin_scripts())),
                        html::custom("body", body), // tuple to force `include_hash`
                    ),
                )
                .render(r, false)
                .await
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
            let (parts, body) = Response::from(err).into_parts();
            return Response::from_parts(parts, Full::new(body));
        }
    };

    let html = result.end().view;
    Response::builder()
        .header(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        )
        .body(Full::new(Bytes::from(format!(
            "{html}\n\
            <script type=\"application/json\" id=\"state\">{scope}</script>"
        ))))
        .unwrap()
}

pub async fn put_page<F: Future<Output = V>, V: View>(
    event: Event,
    render_fn: impl FnOnce() -> F + Send + Sync + 'static,
) -> Response<Full<Bytes>> {
    let (scope, result) = local_pool::spawn(move || async move {
        let scope = Scope::new()
            .with_event(event.event_id, event.payload)
            .with_prev_state(event.state);
        let result = scope
            .clone()
            .run(async move {
                let r = Renderer::new();
                render_fn().await.render(r, true).await
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
            let (parts, body) = Response::from(err).into_parts();
            return Response::from_parts(parts, Full::new(body));
        }
    };

    let html = result.end().view;
    Response::builder()
        .header(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        )
        .body(Full::new(Bytes::from(format!(
            "{html}\n\
            <script type=\"application/json\" id=\"state\">{scope}</script>"
        ))))
        .unwrap()
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
