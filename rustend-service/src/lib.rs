use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{header, Method, Request, Response, StatusCode};
use http_body::Body as HttpBody;
use hyper::body::to_bytes;
use hyper::Body;
use mime::Mime;
use rustend::component::registry::ComponentRegistry;
use rustend::{render, View, SERVER_COMPONENT_JS};
use tower_service::Service;

pub struct RustendService<A, F, V> {
    app: Arc<A>,
    marker: PhantomData<(F, V)>,
}

pub fn app<A, F, V>(app: A) -> RustendService<A, F, V>
where
    A: Fn() -> F + Send + 'static,
    F: Future<Output = V> + Send + 'static,
    V: View + Send + 'static,
{
    RustendService {
        app: Arc::new(app),
        marker: PhantomData,
    }
}

impl<A, F, V, ReqBody> Service<Request<ReqBody>> for RustendService<A, F, V>
where
    A: Fn() -> F + Send + Sync + 'static,
    F: Future<Output = V> + Send + 'static,
    V: View + Send + 'static,
    ReqBody: HttpBody<Data = Bytes> + Send + 'static,
    Error: From<<ReqBody as HttpBody>::Error>,
{
    type Response = Response<Body>;
    type Error = Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let app = self.app.clone();
        Box::pin(async move {
            let path = cleanup_path(req.uri().path());
            let method = req.method();
            // TODO: skip Vec allocation?
            let segments = if path.is_empty() {
                Vec::new()
            } else {
                path.split('/').collect()
            };

            match (method, segments.as_slice()) {
                (&Method::GET, &["server-component.js"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/javascript")
                    .body(SERVER_COMPONENT_JS.into())?),

                #[cfg(feature = "rustend-css")]
                (&Method::GET, &["styles.css"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/css")
                    .body(
                        rustend_css::registry::StyleRegistry::global()
                            .style_sheet()
                            .into(),
                    )?),

                (&Method::GET, &[]) => {
                    // Ensure that component registry has been initialized
                    let _ = ComponentRegistry::global();

                    let view = (app)().await;
                    let html = render(view).await.unwrap();
                    const STYLESHEET: &str = r#"<link rel="stylesheet" href="/styles.css">"#;
                    const SCRIPT: &str = r#"<script src="/server-component.js" async></script>"#;
                    let html = format!("{STYLESHEET}\n{SCRIPT}\n{html}");
                    Ok(Response::builder()
                        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
                        .body(html.into())?)
                }

                (&Method::POST, &["dispatch", component, action]) => {
                    // TODO: get rid of to_string()
                    let id = component.to_string();
                    let action = action.to_string();

                    let mime: Option<Mime> = req
                        .headers()
                        .get(header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok()?.parse().ok());
                    if mime != Some(mime::APPLICATION_JSON) {
                        return Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::empty())?);
                    }

                    let data = to_bytes(req.into_body()).await?;
                    let update = ComponentRegistry::global().handle(&id, &action, data).await;

                    match update {
                        Some(update) => {
                            let json = serde_json::to_vec(&update)?;
                            Ok(Response::builder()
                                .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
                                .body(json.into())?)
                        }
                        None => Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::empty())?),
                    }
                }

                _ => Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())?),
            }
        })
    }
}

impl<A, F, V> Clone for RustendService<A, F, V> {
    fn clone(&self) -> Self {
        RustendService {
            app: self.app.clone(),
            marker: PhantomData,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error("failed to serialize update")]
    Ser(#[from] serde_json::Error),
}

fn cleanup_path(segment: &str) -> &str {
    // remove leading `/`
    let segment = segment.strip_prefix('/').unwrap_or(segment);
    // remove trailing `/`
    let segment = segment.strip_suffix('/').unwrap_or(segment);

    segment
}
