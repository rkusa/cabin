use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use bytes::Bytes;
use http::{HeaderValue, Request, Response, StatusCode};
use http_body::Body;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::InternalError;
use crate::render::{Out, Renderer};
use crate::scope::Scope;
use crate::server::{err_to_response, parse_body};
use crate::view::RenderFuture;
use crate::view::boundary::BoundaryRef;
use crate::{Error, Exposed, View};

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
        Args: Clone + Serialize + DeserializeOwned + Exposed + Send + Sync,
    {
        self.handler.insert(
            boundary.id,
            Arc::new(move |args_json: &str, r: Renderer| {
                let args: Args = match serde_json::from_str(args_json) {
                    Ok(args) => args,
                    Err(err) => {
                        return RenderFuture::Ready(Err(bad_request(InternalError::Deserialize {
                            what: "boundary state json",
                            err: Box::new(err),
                        })));
                    }
                };
                RenderFuture::Future(Box::pin(async move {
                    // The args come from the client, validate them before running the boundary.
                    args.validate().await?;
                    crate::view::FutureExt::into_any_view(boundary.with(args))
                        .render(r)
                        .await
                }))
            }),
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
                Err(err) => return err_to_response(bad_request(err)),
            };

            let mut scope = match Scope::new(true, false)
                .with_validated_event(event.event_id, event.payload)
                .await
            {
                Ok(scope) => scope,
                Err(err) => return err_to_response(err),
            };
            if let Some(multipart) = event.multipart {
                scope = scope.with_multipart(multipart);
            }
            let r = scope.create_renderer();
            let result = scope
                .run(async move { handler(state_json.get(), r).await })
                .await;
            let Out { html, headers } = match result.map(|r| r.end()) {
                Ok(result) => result,
                Err(err) => return err_to_response(err),
            };
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

fn bad_request(err: InternalError) -> Error {
    Error::from(err).with_status(StatusCode::BAD_REQUEST)
}
