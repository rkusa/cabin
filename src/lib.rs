#![forbid(unsafe_code)]
#![feature(return_position_impl_trait_in_trait)]

extern crate self as cabin;

use std::collections::HashMap;
use std::future::Future;
use std::pin::pin;

use bytes::Bytes;
pub use cabin_macros::{element, Attribute};
pub use error::Error;
use futures_util::stream::TryStreamExt;
pub use http::StatusCode;
use http::{HeaderValue, Request, Response};
use http_body::{Body, Full};
use http_error::HttpError;
use mime::Mime;
use multer::Multipart;
use render::{Out, Renderer};
use scope::{Payload, Scope};
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
pub mod serde;
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
        html::script(html::script::src("/server-component.js").defer(), ()),
        #[cfg(feature = "livereload")]
        html::script(html::script::src("/livereload.js").defer(), ()),
    )
}

pub struct Event {
    event_id: u32,
    state: HashMap<StateId, Box<RawValue>>,
    payload: Payload,
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
            if err.status_code().is_server_error() {
                tracing::error!(
                    %err,
                    caused_by = format_caused_by(std::error::Error::source(&err)),
                    "server error",
                );
            } else if err.status_code().is_client_error() {
                tracing::debug!(
                    %err,
                    caused_by = format_caused_by(std::error::Error::source(&err)),
                    "client error",
                );
            }
            let (parts, body) = Response::from(err).into_parts();
            return Response::from_parts(parts, Full::new(body));
        }
    };

    let Out { html, headers } = result.end();
    let mut res = Response::builder().header(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    for (key, value) in headers {
        if let Some(key) = key {
            res = res.header(key, value);
        }
    }
    res.body(Full::new(Bytes::from(format!(
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

pub async fn put_page<F: Future<Output = V>, V: View, B>(
    req: Request<B>,
    render_fn: impl FnOnce() -> F + Send + Sync + 'static,
) -> Response<Full<Bytes>>
where
    B: Body<Data = Bytes> + Send + Sync,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    let event = match parse_body(req).await {
        Ok(result) => result,
        Err(err) => {
            if err.status_code().is_server_error() {
                tracing::error!(
                    %err,
                    caused_by = format_caused_by(std::error::Error::source(&err)),
                    "server error",
                );
            } else if err.status_code().is_client_error() {
                tracing::debug!(
                    %err,
                    caused_by = format_caused_by(std::error::Error::source(&err)),
                    "client error",
                );
            }
            let (parts, body) = Response::from(err).into_parts();
            return Response::from_parts(parts, Full::new(body));
        }
    };
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
            if err.status_code().is_server_error() {
                tracing::error!(
                    %err,
                    caused_by = format_caused_by(std::error::Error::source(&err)),
                    "server error",
                );
            } else if err.status_code().is_client_error() {
                tracing::debug!(
                    %err,
                    caused_by = format_caused_by(std::error::Error::source(&err)),
                    "client error",
                );
            }
            let (parts, body) = Response::from(err).into_parts();
            return Response::from_parts(parts, Full::new(body));
        }
    };

    let Out { html, headers } = result.end();
    let mut res = Response::builder().header(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    for (key, value) in headers {
        if let Some(key) = key {
            res = res.header(key, value);
        }
    }
    res.body(Full::new(Bytes::from(format!(
        "{html}\n\
            <script type=\"application/json\" id=\"state\">{scope}</script>"
    ))))
    .unwrap()
}

async fn parse_body<B>(req: Request<B>) -> Result<Event, Error>
where
    B: Body<Data = Bytes> + Send + Sync,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    // TODO: content-length protection?
    let mime_type: Mime = req
        .headers()
        .get(http::header::CONTENT_TYPE)
        .cloned()
        .and_then(|v| v.to_str().ok()?.parse().ok())
        .ok_or_else(|| Error::from_status_code(StatusCode::UNSUPPORTED_MEDIA_TYPE))?;

    let body = pin!(req.into_body());
    let body = futures_util::stream::try_unfold(body, |mut body| async move {
        let Some(bytes) = body.data().await else {
            return Ok::<_, B::Error>(None);
        };
        Ok(Some((bytes?, body)))
    });

    if mime_type == mime::APPLICATION_JSON {
        #[derive(::serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct JsonEvent {
            event_id: u32,
            state: HashMap<StateId, Box<RawValue>>,
            payload: Box<RawValue>,
        }

        let whole_body = body
            .try_fold(Vec::new(), |mut data, chunk| async move {
                data.extend_from_slice(&chunk);
                Ok(data)
            })
            .await
            .map_err(|err| Error::from_err(err).with_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        let event: JsonEvent = serde_json::from_slice(&whole_body)
            .map_err(|err| Error::from_err(err).with_status(StatusCode::BAD_REQUEST))?;
        Ok(Event {
            event_id: event.event_id,
            state: event.state,
            payload: Payload::Json(event.payload),
        })
    } else if let Ok(boundary) = multer::parse_boundary(mime_type) {
        let mut multi_part = Multipart::new(body, boundary);
        let mut event_id: Option<u32> = None;
        let mut state: Option<HashMap<StateId, Box<RawValue>>> = None;
        let mut payload: Option<String> = None;
        while let Some(field) = multi_part
            .next_field()
            .await
            .map_err(|err| Error::from_err(err).with_status(StatusCode::INTERNAL_SERVER_ERROR))?
        {
            match field.name() {
                Some("event_id") => {
                    event_id = Some(
                        field
                            .text()
                            .await
                            .map_err(|err| {
                                Error::from_err(err)
                                    .with_status(StatusCode::BAD_REQUEST)
                                    .with_reason("payload is not a valid utf8 string")
                            })?
                            .parse()
                            .map_err(|err| {
                                Error::from_err(err)
                                    .with_status(StatusCode::BAD_REQUEST)
                                    .with_reason("event_id is not a valid u32")
                            })?,
                    )
                }
                Some("state") => {
                    state = Some(field.json().await.map_err(|err| {
                        Error::from_err(err)
                            .with_status(StatusCode::BAD_REQUEST)
                            .with_reason("state is not valid json")
                    })?)
                }
                Some("payload") => {
                    payload = Some(field.text().await.map_err(|err| {
                        Error::from_err(err)
                            .with_status(StatusCode::BAD_REQUEST)
                            .with_reason("payload is not a valid utf8 string")
                    })?)
                }
                _ => unimplemented!(),
            }
        }

        Ok(Event {
            event_id: event_id.ok_or_else(|| {
                Error::from_status_code_and_reason(StatusCode::BAD_REQUEST, "event_id missing")
            })?,
            state: state.ok_or_else(|| {
                Error::from_status_code_and_reason(StatusCode::BAD_REQUEST, "state missing")
            })?,
            payload: Payload::UrlEncoded(payload.ok_or_else(|| {
                Error::from_status_code_and_reason(StatusCode::BAD_REQUEST, "payload missing")
            })?),
        })
    } else {
        Err(Error::from_status_code(StatusCode::UNSUPPORTED_MEDIA_TYPE))
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
