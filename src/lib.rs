#![forbid(unsafe_code)]

extern crate self as cabin;

use std::future::Future;
use std::sync::OnceLock;

use bytes::Bytes;
pub use cabin_macros::{boundary, Attribute};
pub use error::Error;
use futures_util::stream::TryStreamExt;
pub use html::h;
pub use http::StatusCode;
use http::{HeaderValue, Request, Response};
use http_body::Body;
use http_body_util::{BodyExt, Full};
use http_error::HttpError;
use mime::Mime;
use multer::Multipart;
pub use redirect::Redirect;
use render::{Out, Renderer};
use scope::{Payload, Scope};
use serde_json::value::RawValue;
pub use view::View;

pub mod error;
pub mod html;
pub mod local_pool;
pub mod multipart;
pub mod prelude;
pub mod private;
mod redirect;
pub mod render;
pub mod scope;
pub mod serde;
pub mod view;

pub const CABIN_JS: &str = include_str!("./cabin.js");
pub const LIVERELOAD_JS: &str = include_str!("./livereload.js");

pub fn cabin_scripts() -> impl View {
    use html::elements::script::Script;

    static SRC_SC: OnceLock<String> = OnceLock::new();
    let src_sc = SRC_SC.get_or_init(|| {
        let hash = content_hash(CABIN_JS.as_bytes());
        format!("/cabin.js?{hash}")
    });

    #[cfg(feature = "livereload")]
    let src_lr = {
        static SRC_LR: OnceLock<String> = OnceLock::new();
        let src_lr = SRC_LR.get_or_init(|| {
            let hash = content_hash(LIVERELOAD_JS.as_bytes());
            format!("/livereload.js?{hash}")
        });
        src_lr
    };

    (
        html::script("").src(src_sc).defer(),
        #[cfg(feature = "livereload")]
        html::script("").src(src_lr).defer(),
    )
}

pub fn content_hash(bytes: &[u8]) -> u32 {
    use std::hash::Hasher;
    let mut hasher = twox_hash::XxHash32::default();
    hasher.write(bytes);
    hasher.finish() as u32
}

pub struct Event {
    pub(crate) event_id: u32,
    pub(crate) state: Option<Box<RawValue>>,
    pub(crate) payload: Payload,
    pub(crate) multipart: Option<Multipart<'static>>,
}

pub fn basic_document(content: impl View) -> impl View {
    (
        html::doctype(),
        html::html((html::head(cabin_scripts()), html::body(content))),
    )
}

pub async fn get_page<F, V>(
    render_fn: impl FnOnce() -> F + Send + Sync + 'static,
) -> Response<Full<Bytes>>
where
    F: Future<Output = V>,
    V: View + 'static,
{
    let result = local_pool::spawn(move || {
        let scope = Scope::new();
        scope.run(async move {
            let r = Renderer::new();
            let doc = render_fn().await;
            doc.render(r, false).await
        })
    })
    .await;
    let result = match result {
        Ok(result) => result,
        Err(err) => return err_to_response(err),
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
    res.body(Full::new(Bytes::from(html))).unwrap()
}

pub async fn put_page<F: Future<Output = V>, V: View, B>(
    req: Request<B>,
    render_fn: impl FnOnce() -> F + Send + 'static,
) -> Response<Full<Bytes>>
where
    B: Body<Data = Bytes> + Send + 'static,
    B::Error: std::error::Error + Send + 'static,
{
    let event = match parse_body(req).await {
        Ok(result) => result,
        Err(err) => return err_to_response(err),
    };
    let result = local_pool::spawn(move || {
        let mut scope = Scope::new().with_event(event.event_id, event.payload);
        if let Some(multipart) = event.multipart {
            scope = scope.with_multipart(multipart);
        }
        scope.run(async move {
            let r = Renderer::new_update();
            render_fn().await.render(r, true).await
        })
    })
    .await;
    let result = match result {
        Ok(result) => result,
        Err(err) => return err_to_response(err),
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
    res.body(Full::new(Bytes::from(html))).unwrap()
}

fn err_to_response(err: Error) -> Response<Full<Bytes>> {
    if err.status_code().is_server_error() {
        if let Some(parent) = err.span() {
            tracing::error!(
                parent: parent,
                %err,
                caused_by = format_caused_by(std::error::Error::source(&err)),
                "server error",
            );
        } else {
            tracing::error!(
                %err,
                caused_by = format_caused_by(std::error::Error::source(&err)),
                "server error",
            );
        }
    } else if err.status_code().is_client_error() {
        if let Some(parent) = err.span() {
            tracing::debug!(
                parent: parent,
                %err,
                caused_by = format_caused_by(std::error::Error::source(&err)),
                "client error",
            );
        } else {
            tracing::debug!(
                %err,
                caused_by = format_caused_by(std::error::Error::source(&err)),
                "client error",
            );
        }
    }
    let (parts, body) = Response::from(err).into_parts();
    Response::from_parts(parts, Full::new(body))
}

async fn parse_body<B>(req: Request<B>) -> Result<Event, Error>
where
    B: Body<Data = Bytes> + Send + 'static,
    B::Error: std::error::Error + Send + 'static,
{
    // TODO: content-length protection?
    let mime_type: Mime = req
        .headers()
        .get(http::header::CONTENT_TYPE)
        .cloned()
        .and_then(|v| v.to_str().ok()?.parse().ok())
        .ok_or_else(|| Error::from_status_code(StatusCode::UNSUPPORTED_MEDIA_TYPE))?;

    let body = Box::pin(req.into_body());
    let body = futures_util::stream::try_unfold(body, |mut body| async move {
        let Some(bytes) = body.frame().await else {
            return Ok::<_, B::Error>(None);
        };
        match bytes?.into_data() {
            Ok(data) => Ok(Some((data, body))),
            Err(_) => Ok(None),
        }
    })
    .map_err(|err| {
        // `multer::Multipart::new` below requires the error to by Sync, to avoid requiring that
        // upstream simply flatten the error to a string here ...
        Box::<dyn std::error::Error + Send + Sync + 'static>::from(err.to_string())
    });

    if mime_type == mime::APPLICATION_JSON {
        #[derive(::serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct JsonEvent {
            event_id: u32,
            state: Option<Box<RawValue>>,
            payload: Box<RawValue>,
        }

        let whole_body = body
            .try_fold(Vec::new(), |mut data, chunk| async move {
                data.extend_from_slice(&chunk);
                Ok(data)
            })
            .await
            .map_err(|err| Error::from(err).with_status(StatusCode::INTERNAL_SERVER_ERROR))?;
        let event: JsonEvent = serde_json::from_slice(&whole_body)
            .map_err(|err| Error::from_err(err).with_status(StatusCode::BAD_REQUEST))?;
        Ok(Event {
            event_id: event.event_id,
            state: event.state,
            payload: Payload::Json(event.payload),
            multipart: None,
        })
    } else if let Ok(boundary) = multer::parse_boundary(mime_type) {
        let mut multipart = Multipart::new(body, boundary);
        let event_id: u32 = {
            let field = multipart.next_field().await.map_err(|err| {
                Error::from_err(err).with_status(StatusCode::INTERNAL_SERVER_ERROR)
            })?;
            let Some(field) = field else {
                return Err(Error::from_status_code_and_reason(
                    StatusCode::BAD_REQUEST,
                    "first multipart field expected to be `event_id`",
                ));
            };
            if field.name() != Some("event_id") {
                return Err(Error::from_status_code_and_reason(
                    StatusCode::BAD_REQUEST,
                    "first multipart field expected to be `event_id`",
                ));
            }
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
                })?
        };
        let state: Option<Box<RawValue>> = {
            let field = multipart.next_field().await.map_err(|err| {
                Error::from_err(err).with_status(StatusCode::INTERNAL_SERVER_ERROR)
            })?;
            let Some(field) = field else {
                return Err(Error::from_status_code_and_reason(
                    StatusCode::BAD_REQUEST,
                    "first multipart field expected to be `state`",
                ));
            };
            if field.name() != Some("state") {
                return Err(Error::from_status_code_and_reason(
                    StatusCode::BAD_REQUEST,
                    "first multipart field expected to be `state`",
                ));
            }
            let data = field
                .bytes()
                .await
                .map_err(|err| Error::from_err(err).with_status(StatusCode::BAD_REQUEST))?;
            if data.is_empty() {
                None
            } else {
                Some(
                    serde_json::from_slice(&data)
                        .map_err(multer::Error::DecodeJson)
                        .map_err(|err| {
                            Error::from_err(err)
                                .with_status(StatusCode::BAD_REQUEST)
                                .with_reason("state is not valid json")
                        })?,
                )
            }
        };
        let payload: String = {
            let field = multipart.next_field().await.map_err(|err| {
                Error::from_err(err).with_status(StatusCode::INTERNAL_SERVER_ERROR)
            })?;
            let Some(field) = field else {
                return Err(Error::from_status_code_and_reason(
                    StatusCode::BAD_REQUEST,
                    "first multipart field expected to be `payload`",
                ));
            };
            if field.name() != Some("payload") {
                return Err(Error::from_status_code_and_reason(
                    StatusCode::BAD_REQUEST,
                    "first multipart field expected to be `payload`",
                ));
            }
            field
                .text()
                .await
                .map_err(|err| Error::from_err(err).with_status(StatusCode::BAD_REQUEST))?
        };

        Ok(Event {
            event_id,
            state,
            payload: Payload::UrlEncoded(payload),
            multipart: Some(multipart),
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
