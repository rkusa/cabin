use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use http::{HeaderValue, Request, Response, StatusCode, header};
use tower_layer::Layer;
use tower_service::Service;

const X_CABIN_REDIRECT: &str = "x-cabin-redirect";

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
        let from_cabin =
            req.headers().get("x-cabin") == Some(&HeaderValue::from_static("boundary"));
        Box::pin(async move {
            let mut res = service.call(req).await?;
            let Some(location) = res.headers_mut().remove(X_CABIN_REDIRECT) else {
                return Ok(res);
            };

            if from_cabin {
                // `fetch` follows redirects on its own (and the `Location` isn't readable when
                // not following it), so send the target in a header for cabin.js to navigate to.
                // Other headers (e.g. `Set-Cookie`) are kept.
                *res.status_mut() = StatusCode::NO_CONTENT;
                *res.body_mut() = Default::default();
                let headers = res.headers_mut();
                headers.remove(header::LOCATION);
                headers.remove(header::CONTENT_TYPE);
                headers.remove(header::CONTENT_LENGTH);
                headers.insert(X_CABIN_REDIRECT, location);
            }
            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::future::{Ready, ready};

    use futures_util::FutureExt;
    use http::header::SET_COOKIE;

    use super::*;

    #[derive(Clone)]
    struct RedirectService;

    impl Service<Request<String>> for RedirectService {
        type Response = Response<String>;
        type Error = Infallible;
        type Future = Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, _req: Request<String>) -> Self::Future {
            let err = cabin::Error::from_http_err(cabin::Redirect::new("https://example.com/auth"));
            let mut res = Response::<String>::from(err);
            res.headers_mut()
                .insert(SET_COOKIE, HeaderValue::from_static("session=1"));
            ready(Ok(res))
        }
    }

    fn call(req: Request<String>) -> Response<String> {
        layer()
            .layer(RedirectService)
            .call(req)
            .now_or_never()
            .unwrap()
            .unwrap()
    }

    #[test]
    fn redirect_for_cabin_request() {
        let req = Request::builder()
            .header("x-cabin", "boundary")
            .body(String::new())
            .unwrap();
        let res = call(req);
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
        assert_eq!(res.headers()[X_CABIN_REDIRECT], "https://example.com/auth");
        assert!(!res.headers().contains_key(header::LOCATION));
        assert_eq!(res.headers()[SET_COOKIE], "session=1");
    }

    #[test]
    fn redirect_for_other_request() {
        let res = call(Request::new(String::new()));
        assert_eq!(res.status(), StatusCode::SEE_OTHER);
        assert_eq!(res.headers()[header::LOCATION], "https://example.com/auth");
        assert!(!res.headers().contains_key(X_CABIN_REDIRECT));
        assert_eq!(res.headers()[SET_COOKIE], "session=1");
    }
}
