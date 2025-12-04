use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use bytes::Bytes;
use http::{HeaderValue, Request, Response, StatusCode};
use http_body::Body;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::View;
use crate::context::Context;
use crate::error::InternalError;
use crate::render::Out;
use crate::server::{err_to_response, parse_body};
use crate::view::RenderFuture;
use crate::view::boundary::BoundaryRef;

type BoundaryHandler = dyn Send + Sync + for<'v> Fn(&'v str, &'v Context) -> RenderFuture<'v>;

#[derive(Default)]
pub struct BoundaryRegistry {
    handler: HashMap<&'static str, Arc<BoundaryHandler>>,
}

impl BoundaryRegistry {
    pub fn add(&mut self, boundaries: &'static [fn(&mut BoundaryRegistry)]) {
        for f in boundaries {
            (f)(self);
        }
    }

    pub fn register<Args>(&mut self, boundary: &'static BoundaryRef<Args>)
    where
        Args: Clone + Serialize + DeserializeOwned,
    {
        self.handler.insert(
            boundary.id,
            Arc::new(
                |args_json: &str, c: &Context| match serde_json::from_str(args_json) {
                    Ok(args) => crate::view::FutureExt::into_view(boundary.with(c, args))
                        .render(c.acquire_renderer()),
                    Err(err) => RenderFuture::Ready(Some(Err(InternalError::Deserialize {
                        what: "boundary state json".into(),
                        err: Box::new(err),
                    }
                    .into()))),
                },
            ),
        );
    }

    pub fn handle<B>(&self, id: &str, req: Request<B>) -> impl Future<Output = Response<String>>
    where
        B: Body<Data = Bytes> + Send + 'static,
        B::Error: std::error::Error + Send + 'static,
    {
        let handler = self.handler.get(id).cloned();

        async move {
            let Some(handler) = handler else {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(String::new())
                    .unwrap();
            };

            let mut event = match parse_body(req).await {
                Ok(result) => result,
                Err(err) => return err_to_response(err),
            };

            let result = event
                .state
                .take()
                .ok_or_else(|| InternalError::Deserialize {
                    what: "boundary state json".into(),
                    err: Box::from("missing boundary state"),
                });
            let state_json = match result {
                Ok(result) => result,
                Err(err) => return err_to_response(err.into()),
            };

            let mut context = Context::new(true).with_event(event.event_id, event.payload);
            if let Some(multipart) = event.multipart {
                context = context.with_multipart(multipart);
            }
            let result = match handler(state_json.get(), &context).await {
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
            res.body(html).unwrap()
        }
    }
}
