use std::convert::Infallible;
use std::future::{Ready, ready};
use std::task::{Context, Poll};

use bytes::Bytes;
use cabin::CABIN_JS;
use http::{Method, Request, Response, header};
use tokio_util::either::Either;
use tower_layer::Layer;
use tower_service::Service;

pub fn layer() -> AssetsLayer {
    AssetsLayer
}

/// Layer to handle framework specific requests.
#[derive(Clone)]
pub struct AssetsLayer;

/// Service to handle framework specific requests.
#[derive(Clone)]
pub struct AssetsService<S> {
    service: S,
}

impl<S> Layer<S> for AssetsLayer {
    type Service = AssetsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AssetsService { service: inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for AssetsService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    ReqBody: http_body::Body<Data = Bytes>,
    // ReqBody::Error: std::error::Error + Send,
    ResBody: From<Bytes>,
{
    type Response = Response<ResBody>;
    type Error = Infallible;
    type Future = Either<Ready<Result<Self::Response, Self::Error>>, S::Future>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/cabin.js") => Either::Left(ready(Ok(Response::builder()
                .header(header::CONTENT_TYPE, "text/javascript")
                .header(
                    header::CACHE_CONTROL,
                    if req.uri().query().is_some() {
                        "public,max-age=31536000,immutable"
                    } else {
                        "no-cache"
                    },
                )
                .body(Bytes::from(CABIN_JS).into())
                .unwrap()))),

            _ => Either::Right(self.service.call(req)),
        }
    }
}
