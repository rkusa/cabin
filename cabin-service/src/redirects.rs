use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{HeaderValue, Method, Request, Response, StatusCode, header};
use tower_layer::Layer;
use tower_service::Service;

pub fn layer() -> RedirectsLayer {
    RedirectsLayer
}

/// Layer to handle framework specific requests.
#[derive(Clone)]
pub struct RedirectsLayer;

/// Service to handle framework specific requests.
#[derive(Clone)]
pub struct RedirectsService<S> {
    service: S,
}

impl<S> Layer<S> for RedirectsLayer {
    type Service = RedirectsService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RedirectsService { service: inner }
    }
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for RedirectsService<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: std::marker::Send,
    ReqBody: http_body::Body<Data = Bytes> + Send + 'static,
    ReqBody::Error: std::error::Error + Send,
    ResBody: http_body::Body<Data = Bytes> + Default,
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
            if req.method() == Method::GET && req.uri().path() == "/client_redirect" {
                if req.headers().get("x-cabin") == Some(&HeaderValue::from_static("boundary")) {
                    Ok(Response::builder()
                        .status(StatusCode::NO_CONTENT)
                        // TODO: handle missing query?
                        // TODO: validate same host for redirect?
                        // TODO: be smarter about client_redirect and not have the extra step
                        // for full       page navigations?
                        // .header(header::LOCATION, req.uri().query().unwrap_or(""))
                        .body(Default::default())
                        .unwrap())
                } else {
                    Ok(Response::builder()
                        .status(StatusCode::SEE_OTHER)
                        // TODO: handle missing query?
                        // TODO: validate same host for redirect?
                        // TODO: be smarter about client_redirect and not have the extra step
                        // for full       page navigations?
                        .header(header::LOCATION, req.uri().query().unwrap_or(""))
                        .body(Default::default())
                        .unwrap())
                }
            } else {
                service.call(req).await
            }
        })
    }
}
