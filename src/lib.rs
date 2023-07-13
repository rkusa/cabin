#![forbid(unsafe_code)]
#![feature(return_position_impl_trait_in_trait)]

extern crate self as cabin;

use std::collections::HashMap;
use std::future::Future;

use bytes::Bytes;
pub use cabin_macros::{element, Attribute};
pub use error::Error;
pub use http::StatusCode;
use http::{HeaderValue, Response};
use http_body::Full;
use http_error::HttpError;
use render::Renderer;
use scope::Scope;
use serde_json::value::RawValue;
use state::StateId;
pub use view::View;

pub mod error;
pub mod html;
mod local_pool;
pub mod prelude;
pub mod private;
pub mod render;
pub mod scope;
pub mod state;
pub mod view;

pub const SERVER_COMPONENT_JS: &str = include_str!("./server-component.js");
pub const LIVERELOAD_JS: &str = include_str!("./livereload.js");

// TODO: move behind feature flag?
pub fn cabin_stylesheets() -> impl View {
    use html::elements::link::Link;
    html::link(
        html::id("cabin-styles")
            .rel(html::elements::link::Rel::StyleSheet)
            .href("/styles.css"),
    )
}

pub fn cabin_scripts() -> impl View {
    use html::elements::script::Script;
    (
        html::script(html::script::src("/server-component.js").r#async(), ()),
        #[cfg(feature = "livereload")]
        html::script(html::script::src("/livereload.js").r#async(), ()),
    )
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    event_id: u32,
    state: HashMap<StateId, Box<RawValue>>,
    payload: Box<RawValue>,
}

fn default_document(content: impl View) -> impl View {
    (
        html::doctype(),
        html::html(
            (),
            (
                html::head((), (cabin_stylesheets(), cabin_scripts())),
                html::body((), content),
            ),
        ),
    )
}

pub async fn get_page_with<F, V, D>(
    render_fn: impl FnOnce() -> F + Send + Sync + 'static,
    document: impl FnOnce((V,)) -> D + Send + Sync + 'static,
) -> Response<Full<Bytes>>
where
    F: Future<Output = V>,
    V: View + 'static,
    D: View,
{
    let (scope, result) = local_pool::spawn(move || async move {
        let scope = Scope::new();
        let result = scope
            .clone()
            .run(async move {
                let r = Renderer::new();
                let body = render_fn().await;
                let doc = (document)((
                    body,
                    // tuple to force `include_hash`
                ));
                doc.render(r, false).await
            })
            .await;
        (scope.into_view(), result)
    })
    .await;
    let result = match result {
        Ok(result) => result,
        Err(err) => {
            if err.status_code().is_success() {
                tracing::error!(
                    %err,
                    caused_by = format_caused_by(std::error::Error::source(&err)),
                    "server error",
                );
            }
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

pub async fn get_page<F: Future<Output = V>, V: View + 'static>(
    render_fn: impl FnOnce() -> F + Send + Sync + 'static,
) -> Response<Full<Bytes>> {
    get_page_with(render_fn, default_document).await
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
