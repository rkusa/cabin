use std::any::{Any, TypeId};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

use bytes::Bytes;
use http::{HeaderValue, Request, Response, StatusCode};
use http_body::{Body, Full};
use http_error::HttpError;
use once_cell::race::OnceBox;
use serde::de::DeserializeOwned;
use serde::Serialize;

use super::{BoxedView, RenderFuture};
use crate::error::InternalError;
use crate::html::attributes::Attributes;
use crate::html::script::{self, script};
use crate::html::Html;
use crate::render::{ElementRenderer, Out, Renderer};
use crate::scope::Scope;
use crate::{err_to_response, local_pool, parse_body, View};

type BoundaryFn<Args> = dyn Send + Sync + Fn(Args) -> Pin<Box<dyn Future<Output = Boundary<Args>>>>;

pub struct BoundaryRef<Args>
where
    Args: 'static,
{
    id: &'static str,
    events: &'static [TypeId],
    args: PhantomData<Args>,
    f: &'static BoundaryFn<Args>,
}

#[derive(Default)]
pub struct BoundaryEvent<E> {
    marker: PhantomData<E>,
}

impl<Args> BoundaryRef<Args>
where
    Args: 'static,
{
    pub const fn new(
        id: &'static str,
        events: &'static [TypeId],
        f: &'static BoundaryFn<Args>,
    ) -> Self {
        Self {
            id,
            events,
            args: PhantomData,
            f,
        }
    }

    async fn with(&'static self, args: Args) -> Boundary<Args> {
        self::internal::Boundary::upgrade((self.f)(args).await, self).into_update()
    }
}

pub const fn type_id<T: 'static + ?Sized>() -> TypeId {
    TypeId::of::<T>()
}

pub struct Boundary<Args>
where
    Args: 'static,
{
    boundary_ref: Option<&'static BoundaryRef<Args>>,
    // TODO: take reference to args to avoid cloning them?
    args: Option<Args>,
    view: BoxedView,
    #[allow(clippy::type_complexity)]
    prerender: Option<Vec<(u32, Result<String, InternalError>, Box<dyn Any>)>>,
    is_update: bool,
}

impl<Args> Boundary<Args> {
    pub(crate) fn new(view: impl View, args: Args) -> Self {
        Boundary {
            boundary_ref: None,
            args: Some(args),
            view: view.boxed(),
            prerender: None,
            is_update: false,
        }
    }

    pub fn prerender<E: Serialize + 'static>(mut self, event: E) -> Self {
        let mut hasher = twox_hash::XxHash32::default();
        std::any::TypeId::of::<E>().hash(&mut hasher);
        let event_id = hasher.finish() as u32;

        let json = serde_json::to_string(&event).map_err(|err| InternalError::Serialize {
            what: "on_click event",
            err,
        });

        // TODO: use `get_or_insert_default` once stable
        if self.prerender.is_none() {
            self.prerender = Some(vec![(event_id, json, Box::new(event))]);
        } else {
            self.prerender
                .as_mut()
                .unwrap()
                .push((event_id, json, Box::new(event)));
        }

        self
    }
}

pub mod internal {
    pub use super::*;

    pub trait Boundary<Args> {
        fn upgrade(self, id: &'static BoundaryRef<Args>) -> Self;
    }

    impl<Args> Boundary<Args> for super::Boundary<Args> {
        fn upgrade(mut self, boundary_ref: &'static BoundaryRef<Args>) -> Self {
            self.boundary_ref = Some(boundary_ref);
            self
        }
    }

    impl<Args, E> Boundary<Args> for Result<super::Boundary<Args>, E> {
        fn upgrade(mut self, boundary_ref: &'static BoundaryRef<Args>) -> Self {
            if let Ok(b) = &mut self {
                b.boundary_ref = Some(boundary_ref);
            }
            self
        }
    }
}

impl<Args> Boundary<Args>
where
    Args: 'static,
{
    fn into_update(mut self) -> Self {
        self.is_update = true;
        self
    }
}

impl<Args> View for Boundary<Args>
where
    Args: 'static + Clone + Serialize,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        let Some(args) = self.args else {
            return self.view.render(r, include_hash);
        };

        // TODO: any way to make this a compile error?
        let Some(boundary_ref) = self.boundary_ref else {
            return RenderFuture::Ready(Some(Err(InternalError::MissingBoundaryAttribute.into())));
        };

        let state = match serde_json::to_string(&args) {
            Ok(state) => state,
            Err(err) => {
                return RenderFuture::Ready(Some(Err(InternalError::Serialize {
                    what: "boundary state",
                    err,
                }
                .into())))
            }
        };

        let preprender = self.prerender.map(move |prerender| async move {
            let mut templates = Vec::with_capacity(prerender.len());
            for (event_id, json, event) in prerender {
                let scope = Scope::new().with_deserialized_event(event);

                let args = args.clone();
                let boundary = scope
                    .run(async move { Ok(boundary_ref.with(args).await) })
                    .await?;

                let Some(args) = boundary.args else {
                    continue;
                };

                let state = match serde_json::to_string(&args) {
                    Ok(state) => state,
                    Err(err) => {
                        return Err(InternalError::Serialize {
                            what: "boundary state",
                            err,
                        }
                        .into())
                    }
                };

                templates.push(
                    Html::new(
                        "template",
                        ("event-id", Cow::Owned(event_id.to_string()))
                            .with(("event-payload", Cow::Owned(json?))),
                        (
                            script(script::r#type("application/json"), state),
                            boundary.view,
                        ),
                    )
                    .boxed(),
                )
            }

            Ok::<Vec<BoxedView>, crate::Error>(templates)
        });

        if let Some(preprender) = preprender {
            RenderFuture::Future(Box::pin(async move {
                let templates = preprender.await?;

                let body = (
                    script(script::r#type("application/json"), state),
                    self.view,
                    #[allow(clippy::map_identity)]
                    templates.into_iter().map(|t| t),
                );
                if self.is_update {
                    body.render(r, include_hash).await
                } else {
                    Html::new("cabin-boundary", boundary_ref, body)
                        .render(r, include_hash)
                        .await
                }
            }))
        } else {
            let body = (script(script::r#type("application/json"), state), self.view);
            if self.is_update {
                body.render(r, include_hash)
            } else {
                Html::new("cabin-boundary", boundary_ref, body).render(r, include_hash)
            }
        }
    }
}

impl<Args> Attributes for &'static BoundaryRef<Args> {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        r.attribute("name", self.id).map_err(InternalError::from)?;
        r.attribute("events", EventsList(self.events))
            .map_err(InternalError::from)?;
        Ok(())
    }
}

#[derive(Hash)]
struct EventsList(&'static [TypeId]);

impl fmt::Display for EventsList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, ev) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }

            let mut hasher = twox_hash::XxHash32::default();
            ev.hash(&mut hasher);
            let hash = hasher.finish() as u32;
            write!(f, "{}", hash)?;
        }

        Ok(())
    }
}

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

impl<Args, E> From<Result<Boundary<Args>, E>> for Boundary<Args>
where
    Args: 'static + Clone + Serialize,
    Box<dyn HttpError + Send + 'static>: From<E>,
    E: 'static,
{
    fn from(result: Result<Boundary<Args>, E>) -> Self {
        match result {
            Ok(b) => b,
            Err(err) => Boundary {
                boundary_ref: None,
                args: None,
                view: View::boxed(Err::<Boundary<Args>, _>(err)),
                prerender: None,
                is_update: false,
            },
        }
    }
}
