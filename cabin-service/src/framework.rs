use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use cabin::CABIN_JS;
use http::{header, HeaderValue, Method, Request, Response, StatusCode};
use http_body::Body as HttpBody;
use http_body_util::combinators::UnsyncBoxBody;
use http_body_util::{BodyExt, Empty, Full};
use tower_layer::Layer;
use tower_service::Service;

/// Layer to handle framework specific requests.
#[derive(Clone)]
pub struct FrameworkLayer;

/// Service to handle framework specific requests.
#[derive(Clone)]
pub struct FrameworkService<S> {
    service: S,
}

impl<S> Layer<S> for FrameworkLayer {
    type Service = FrameworkService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        FrameworkService { service: inner }
    }
}

impl<S, ReqBody, ResBody, ResBodyError> Service<Request<ReqBody>> for FrameworkService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    <S as Service<Request<ReqBody>>>::Error: Into<Infallible> + 'static,
    <S as Service<Request<ReqBody>>>::Future: Send + 'static,
    ReqBody: HttpBody<Data = Bytes> + Send + 'static,
    <ReqBody as HttpBody>::Error: std::error::Error + Send,
    ResBody: HttpBody<Data = Bytes, Error = ResBodyError> + Send + 'static,
{
    type Response = Response<UnsyncBoxBody<Bytes, ResBodyError>>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        #[cfg(feature = "cabin-tailwind")]
        let _ = cabin_tailwind::registry::StyleRegistry::style_sheet();

        let mut service = self.service.clone();
        Box::pin(async move {
            let path = cleanup_path(req.uri().path());
            let method = req.method();

            // Don't have more than 3 segements, so take up to four so 3 segments don't match just
            // because the rest was ignored.
            let mut segments = [""; 4];
            let mut len = 0;
            for (i, segment) in path.split('/').enumerate() {
                let Some(v) = segments.get_mut(i) else {
                    break;
                };
                *v = segment;
                len += 1;
            }

            match (method, &segments[..len]) {
                (&Method::GET, &["cabin.js"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/javascript")
                    .header(
                        header::CACHE_CONTROL,
                        if req.uri().query().is_some() {
                            "public,max-age=31536000,immutable"
                        } else {
                            "no-cache"
                        },
                    )
                    .body(UnsyncBoxBody::new(
                        Full::new(Bytes::from(CABIN_JS)).map_err(|_| unreachable!()),
                    ))
                    .unwrap()),

                #[cfg(feature = "cabin-tailwind")]
                (&Method::GET, &["styles.css"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/css")
                    .header(
                        header::CACHE_CONTROL,
                        if req.uri().query().is_some() {
                            "public,max-age=31536000,immutable"
                        } else {
                            "no-cache"
                        },
                    )
                    .body(UnsyncBoxBody::new(
                        Full::new(Bytes::from(
                            cabin_tailwind::registry::StyleRegistry::style_sheet(),
                        ))
                        .map_err(|_| unreachable!()),
                    ))
                    .unwrap()),

                #[cfg(feature = "livereload")]
                (&Method::GET, &["livereload.js"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/javascript")
                    .body(UnsyncBoxBody::new(
                        Full::new(Bytes::from(cabin::LIVERELOAD_JS)).map_err(|_| unreachable!()),
                    ))
                    .unwrap()),

                #[cfg(feature = "livereload")]
                (&Method::GET, &["livereload"]) => {
                    // Return an event-stream that is only meant to keep a connection open
                    // (periodically sends a heartbeat).
                    Ok(Response::builder()
                        .header(header::CACHE_CONTROL, "no-store")
                        .header(header::CONTENT_TYPE, "text/event-stream")
                        .body(UnsyncBoxBody::new(
                            crate::livereload::Heartbeat::default().map_err(|_| unreachable!()),
                        ))
                        .unwrap())
                }

                (&Method::PUT, &["__boundary", id]) => {
                    let id = id.to_string();
                    let res = cabin::boundary_registry::BoundaryRegistry::global()
                        .handle(&id, req)
                        .await;
                    let (parts, body) = res.into_parts();
                    Ok(Response::from_parts(
                        parts,
                        body.map_err(|_| unreachable!()).boxed_unsync(),
                    ))
                }

                // TODO: error to help debugging (or redirect again?)
                (&Method::GET, &["client_redirect"]) => {
                    if req.headers().get("x-cabin") == Some(&HeaderValue::from_static("boundary")) {
                        Ok(Response::builder()
                            .status(StatusCode::NO_CONTENT)
                            // TODO: handle missing query?
                            // TODO: validate same host for redirect?
                            // TODO: be smarter about client_redirect and not have the extra step
                            // for full       page navigations?
                            // .header(header::LOCATION, req.uri().query().unwrap_or(""))
                            .body(UnsyncBoxBody::new(Empty::new().map_err(|_| unreachable!())))
                            .unwrap())
                    } else {
                        Ok(Response::builder()
                            .status(StatusCode::SEE_OTHER)
                            // TODO: handle missing query?
                            // TODO: validate same host for redirect?
                            // TODO: be smarter about client_redirect and not have the extra step
                            // for full       page navigations?
                            .header(header::LOCATION, req.uri().query().unwrap_or(""))
                            .body(UnsyncBoxBody::new(Empty::new().map_err(|_| unreachable!())))
                            .unwrap())
                    }
                }

                _ => service.call(req).await.map_err(Into::into).map(|r| {
                    let (parts, body) = r.into_parts();
                    Response::from_parts(parts, body.boxed_unsync())
                }),
            }
        })
    }
}

fn cleanup_path(segment: &str) -> &str {
    // remove leading `/`
    let segment = segment.strip_prefix('/').unwrap_or(segment);
    // remove trailing `/`
    let segment = segment.strip_suffix('/').unwrap_or(segment);

    segment
}
