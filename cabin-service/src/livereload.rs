use std::convert::Infallible;
use std::future::{ready, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::Bytes;
use futures_util::future::MapOk;
use futures_util::TryFutureExt;
use http::{header, Method, Request, Response};
use http_body::{Body, Frame};
use http_body_util::combinators::UnsyncBoxBody;
use http_body_util::Full;
use tokio::time::{interval, Interval};
use tokio_util::either::Either;
use tower_layer::Layer;
use tower_service::Service;

pub fn layer() -> LivereloadLayer {
    LivereloadLayer
}

pub struct Heartbeat {
    interval: Interval,
}

impl Body for Heartbeat {
    type Data = Bytes;
    type Error = Infallible;

    #[inline]
    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.as_mut().interval.poll_tick(cx) {
            Poll::Ready(_) => Poll::Ready(Some(Ok(Frame::data(Bytes::from(": heartbeat"))))),
            Poll::Pending => Poll::Pending,
        }
    }

    #[inline]
    fn is_end_stream(&self) -> bool {
        false
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self {
            interval: interval(Duration::from_secs(10)),
        }
    }
}

/// Layer to handle framework specific requests.
#[derive(Clone)]
pub struct LivereloadLayer;

/// Service to handle framework specific requests.
#[derive(Clone)]
pub struct LivereloadService<S> {
    service: S,
}

impl<S> Layer<S> for LivereloadLayer {
    type Service = LivereloadService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LivereloadService { service: inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for LivereloadService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,

    ReqBody: http_body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: std::error::Error + Send,
    ResBody: http_body::Body<Data = Bytes>,
{
    type Response = Response<http_body_util::Either<UnsyncBoxBody<Bytes, Infallible>, ResBody>>;
    type Error = Infallible;
    type Future = Either<
        Ready<Result<Self::Response, Self::Error>>,
        MapOk<S::Future, fn(Response<ResBody>) -> Self::Response>,
    >;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/livereload.js") => Either::Left(ready(Ok(Response::builder()
                .header(header::CONTENT_TYPE, "text/javascript")
                .body(http_body_util::Either::Left(UnsyncBoxBody::new(Full::new(
                    Bytes::from(cabin::LIVERELOAD_JS),
                ))))
                .unwrap()))),

            (&Method::GET, "/livereload") => {
                // Return an event-stream that is only meant to keep a connection open
                // (periodically sends a heartbeat).
                Either::Left(ready(Ok(Response::builder()
                    .header(header::CACHE_CONTROL, "no-store")
                    .header(header::CONTENT_TYPE, "text/event-stream")
                    .body(http_body_util::Either::Left(UnsyncBoxBody::new(
                        crate::livereload::Heartbeat::default(),
                    )))
                    .unwrap())))
            }

            _ => Either::Right(self.service.call(req).map_ok(map_body_to_boxed_unsync)),
        }
    }
}

fn map_body_to_boxed_unsync<B>(
    res: Response<B>,
) -> Response<http_body_util::Either<UnsyncBoxBody<Bytes, Infallible>, B>>
where
    B: http_body::Body<Data = Bytes>,
{
    let (parts, body) = res.into_parts();
    Response::from_parts(parts, http_body_util::Either::Right(body))
}
