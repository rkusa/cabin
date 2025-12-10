use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use bytes::Bytes;
use http::{HeaderValue, Request, Response, StatusCode};
use http_body::Body;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::View;
use crate::error::InternalError;
use crate::render::{Out, Renderer};
use crate::scope::Scope;
use crate::server::{err_to_response, parse_body};
use crate::view::RenderFuture;
use crate::view::boundary::BoundaryRef;

type BoundaryHandler = dyn Send + Sync + Fn(&str, Renderer) -> RenderFuture;

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
        Args: Clone + Serialize + DeserializeOwned + Send + Sync,
    {
        self.handler.insert(
            boundary.id,
            Arc::new(
                |args_json: &str, r: Renderer| match serde_json::from_str(args_json) {
                    Ok(args) => {
                        crate::view::FutureExt::into_view(boundary.with(args)).render(r, true)
                    }
                    Err(err) => RenderFuture::Ready(Err(InternalError::Deserialize {
                        what: "boundary state json",
                        err: Box::new(err),
                    }
                    .into())),
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
                    what: "boundary state json",
                    err: Box::from("missing boundary state"),
                });
            let state_json = match result {
                Ok(result) => result,
                Err(err) => return err_to_response(err.into()),
            };

            let mut scope = Scope::new().with_event(event.event_id, event.payload);
            if let Some(multipart) = event.multipart {
                scope = scope.with_multipart(multipart);
            }
            let result = scope
                .run(async move {
                    let r = Renderer::new_update();
                    handler(state_json.get(), r).await
                })
                .await;
            let result = match result {
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
