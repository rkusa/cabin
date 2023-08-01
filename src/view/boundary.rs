use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use cabin_macros::Attribute;
use http::{HeaderValue, Request, Response, StatusCode};
use http_body::{Body, Full};
use once_cell::race::OnceBox;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::{BoxedView, RenderFuture};
use crate::error::InternalError;
use crate::html::script::{self, script};
use crate::html::Html;
use crate::render::{Out, Renderer};
use crate::scope::Scope;
use crate::{err_to_response, local_pool, parse_body, View};

pub fn boundary<Args>(args: Args, view: impl View) -> Boundary<Args> {
    Boundary {
        id: None,
        args,
        view: view.boxed(),
        is_update: false,
    }
}

type BoundaryFn<Args> = dyn Send + Sync + Fn(Args) -> Pin<Box<dyn Future<Output = Boundary<Args>>>>;

pub struct BoundaryRef<Args>
where
    Args: 'static,
{
    id: &'static str,
    args: PhantomData<Args>,
    f: &'static BoundaryFn<Args>,
}

impl<Args> BoundaryRef<Args>
where
    Args: 'static,
{
    pub const fn new(id: &'static str, f: &'static BoundaryFn<Args>) -> Self {
        Self {
            id,
            args: PhantomData,
            f,
        }
    }

    async fn with(&'static self, args: Args) -> Boundary<Args> {
        (self.f)(args).await.with_id(self.id).into_update()
    }
}

pub struct Boundary<Args>
where
    Args: 'static,
{
    id: Option<&'static str>,
    // TODO: take reference to args to avoid cloning them?
    args: Args,
    view: BoxedView,
    is_update: bool,
}

impl<Args> Boundary<Args>
where
    Args: 'static,
{
    pub fn with_id(mut self, id: &'static str) -> Self {
        self.id = Some(id);
        self
    }

    fn into_update(mut self) -> Self {
        self.is_update = true;
        self
    }
}

impl<Args> View for Boundary<Args>
where
    Args: 'static + Serialize,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        // TODO: any way to make this a compile error?
        let Some(id) = self.id else {
            return RenderFuture::Ready(Some(Err(InternalError::MissingBoundaryAttribute.into())));
        };

        let state = match serde_json::to_string(&self.args) {
            Ok(state) => state,
            Err(err) => {
                return RenderFuture::Ready(Some(Err(InternalError::Serialize {
                    what: "boundary state",
                    err,
                }
                .into())))
            }
        };

        let body = (script(script::r#type("application/json"), state), self.view);
        if self.is_update {
            body.render(r, include_hash)
        } else {
            Html::new("cabin-boundary", Name(id), body).render(r, include_hash)
        }
    }
}

// TODO: hash name in production?
#[derive(Attribute)]
pub struct Name(&'static str);

#[linkme::distributed_slice]
pub static BOUNDARIES: [fn(&mut BoundaryRegistry)] = [..];

static REGISTRY: OnceBox<BoundaryRegistry> = OnceBox::new();

type BoundaryHandler = dyn Send + Sync + Fn(&str, Renderer) -> RenderFuture;

pub struct BoundaryRegistry {
    handler: HashMap<&'static str, Arc<BoundaryHandler>>,
}

impl BoundaryRegistry {
    pub fn global() -> &'static Self {
        REGISTRY.get_or_init(|| {
            let mut registry = Self {
                handler: Default::default(),
            };
            for f in BOUNDARIES {
                (f)(&mut registry);
            }
            Box::new(registry)
        })
    }

    pub fn register<Args>(&mut self, boundary: &'static BoundaryRef<Args>)
    where
        Args: 'static + Clone + Serialize + DeserializeOwned + Send + Sync,
    {
        self.handler.insert(
            boundary.id,
            Arc::new(
                |args_json: &str, r: Renderer| match serde_json::from_str(args_json) {
                    Ok(args) => crate::view::future::FutureExt::into_view(boundary.with(args))
                        .render(r, true),
                    Err(err) => RenderFuture::Ready(Some(Err(InternalError::Deserialize {
                        what: "boundary state json",
                        err: Box::new(err),
                    }
                    .into()))),
                },
            ),
        );
    }

    pub async fn handle<B>(&self, id: &str, req: Request<B>) -> Response<Full<Bytes>>
    where
        B: Body<Data = Bytes> + Send + 'static,
        B::Error: std::error::Error + Send + 'static,
    {
        let Some(handler) = self.handler.get(id) else {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::new()))
                .unwrap();
        };
        let handler = Arc::clone(handler);

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

        let result = local_pool::spawn(move || {
            let scope = Scope::new().with_event(event.event_id, event.payload);
            scope.run(async move {
                let r = Renderer::new();
                handler(state_json.get(), r).await
            })
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
        res.body(Full::new(Bytes::from(html))).unwrap()
    }
}
