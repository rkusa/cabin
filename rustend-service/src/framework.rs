use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::{Buf, BufMut, Bytes};
use http::{header, Method, Request, Response, StatusCode};
use http_body::combinators::UnsyncBoxBody;
use http_body::{Body as HttpBody, Empty, Full};
use mime::Mime;
use rustend::component::registry::ComponentRegistry;
use rustend::SERVER_COMPONENT_JS;
use rustend_css::registry::StyleRegistry;
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
    ReqBody: HttpBody + Send + 'static,
    <ReqBody as HttpBody>::Data: Send,
    <ReqBody as HttpBody>::Error: std::error::Error,
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
        let _ = ComponentRegistry::global();
        let _ = StyleRegistry::global();

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
                (&Method::GET, &["server-component.js"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/javascript")
                    .body(UnsyncBoxBody::new(
                        Full::new(Bytes::from(SERVER_COMPONENT_JS)).map_err(|_| unreachable!()),
                    ))
                    .unwrap()),

                #[cfg(feature = "rustend-css")]
                (&Method::GET, &["styles.css"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/css")
                    .body(UnsyncBoxBody::new(
                        Full::new(Bytes::from(
                            rustend_css::registry::StyleRegistry::global().style_sheet(),
                        ))
                        .map_err(|_| unreachable!()),
                    ))
                    .unwrap()),

                (&Method::POST, &["dispatch", component]) => {
                    // TODO: get rid of to_string()
                    let id = component.to_string();

                    let mime: Option<Mime> = req
                        .headers()
                        .get(header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok()?.parse().ok());
                    if mime != Some(mime::APPLICATION_JSON) {
                        return Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(UnsyncBoxBody::new(Empty::new().map_err(|_| unreachable!())))
                            .unwrap());
                    }

                    let data = match to_bytes(req.into_body()).await {
                        Ok(data) => data,
                        Err(err) => {
                            tracing::error!(%err, "failed to read request body");
                            return Ok(Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(UnsyncBoxBody::new(Empty::new().map_err(|_| unreachable!())))
                                .unwrap());
                        }
                    };
                    let update = match ComponentRegistry::global().handle(&id, data).await {
                        Ok(update) => update,
                        Err(err) => {
                            let res = Response::<Bytes>::from(err);
                            let (parts, body) = res.into_parts();

                            return Ok(Response::from_parts(
                                parts,
                                UnsyncBoxBody::new(Full::new(body).map_err(|_| unreachable!())),
                            ));
                        }
                    };

                    match update {
                        Some(update) => match serde_json::to_vec(&update) {
                            Ok(json) => Ok(Response::builder()
                                .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
                                .body(UnsyncBoxBody::new(
                                    Full::new(Bytes::from(json)).map_err(|_| unreachable!()),
                                ))
                                .unwrap()),
                            Err(err) => {
                                tracing::error!(%err, "failed to serialize action update");
                                Ok(Response::builder()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(UnsyncBoxBody::new(
                                        Empty::new().map_err(|_| unreachable!()),
                                    ))
                                    .unwrap())
                            }
                        },
                        None => Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(UnsyncBoxBody::new(Empty::new().map_err(|_| unreachable!())))
                            .unwrap()),
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

// Taken from hyper, to avoid the dependency
async fn to_bytes<T>(body: T) -> Result<Bytes, T::Error>
where
    T: HttpBody,
{
    let mut body = Box::pin(body); // hyper impl uses futures_util::pin_mut

    // If there's only 1 chunk, we can just return Buf::to_bytes()
    let mut first = if let Some(buf) = body.data().await {
        buf?
    } else {
        return Ok(Bytes::new());
    };

    let second = if let Some(buf) = body.data().await {
        buf?
    } else {
        return Ok(first.copy_to_bytes(first.remaining()));
    };

    // With more than 1 buf, we gotta flatten into a Vec first.
    let cap = first.remaining() + second.remaining() + body.size_hint().lower() as usize;
    let mut vec = Vec::with_capacity(cap);
    vec.put(first);
    vec.put(second);

    while let Some(buf) = body.data().await {
        vec.put(buf?);
    }

    Ok(vec.into())
}
