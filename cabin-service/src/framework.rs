use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use cabin::SERVER_COMPONENT_JS;
use cabin_css::registry::StyleRegistry;
use http::{header, Method, Request, Response};
use http_body::combinators::UnsyncBoxBody;
use http_body::{Body as HttpBody, Full};
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
        // let _ = ComponentRegistry::global();
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

                #[cfg(feature = "cabin-css")]
                (&Method::GET, &["styles.css"]) => Ok(Response::builder()
                    .header(header::CONTENT_TYPE, "text/css")
                    .body(UnsyncBoxBody::new(
                        Full::new(Bytes::from(
                            cabin_css::registry::StyleRegistry::global().style_sheet(),
                        ))
                        .map_err(|_| unreachable!()),
                    ))
                    .unwrap()),

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
