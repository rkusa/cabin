use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{Method, Request, Response};
use tower_layer::Layer;
use tower_service::Service;

pub fn layer() -> BoundariesLayer {
    BoundariesLayer
}

/// Layer to handle framework specific requests.
#[derive(Clone)]
pub struct BoundariesLayer;

/// Service to handle framework specific requests.
#[derive(Clone)]
pub struct BoundariesService<S> {
    service: S,
}

impl<S> Layer<S> for BoundariesLayer {
    type Service = BoundariesService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        BoundariesService { service: inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for BoundariesService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: std::marker::Send,
    ReqBody: http_body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: std::error::Error + Send,
    ResBody: http_body::Body<Data = Bytes> + From<String>,
{
    type Response = Response<ResBody>;
    type Error = Infallible;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let mut service = self.service.clone();
        Box::pin(async move {
            if let Some(id) = (req.method() == Method::PUT)
                .then(|| req.uri().path().strip_prefix("/__boundary/"))
                .flatten()
            {
                let id = id.to_string();
                let res = cabin::boundary_registry::BoundaryRegistry::global()
                    .handle(&id, req)
                    .await;
                let (parts, body) = res.into_parts();
                Ok(Response::from_parts(parts, body.into()))
            } else {
                service.call(req).await
            }
        })
    }
}
