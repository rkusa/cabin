use std::fmt;
use std::future::Future;
use std::hash::Hash;
use std::marker::PhantomData;
use std::pin::Pin;

use http_error::HttpError;
use script::Script;
use serde::Serialize;

use super::{BoxedView, IntoView, RenderFuture};
use crate::View;
use crate::error::InternalError;
use crate::html::Html;
use crate::html::attributes::Attributes;
use crate::html::script::{self, script};
use crate::render::{ElementRenderer, Renderer};

type BoundaryFn<Args> =
    dyn Send + Sync + Fn(Args) -> Pin<Box<dyn Future<Output = Boundary<Args>> + Send>>;

pub struct BoundaryRef<Args>
where
    Args: Send + 'static,
{
    pub id: &'static str,
    events: &'static [&'static str],
    args: PhantomData<Args>,
    f: &'static BoundaryFn<Args>,
}

#[derive(Default)]
pub struct BoundaryEvent<E> {
    marker: PhantomData<E>,
}

impl<Args> BoundaryRef<Args>
where
    Args: Send + 'static,
{
    pub const fn new(
        id: &'static str,
        events: &'static [&'static str],
        f: &'static BoundaryFn<Args>,
    ) -> Self {
        Self {
            id,
            events,
            args: PhantomData,
            f,
        }
    }

    pub async fn with(&'static self, args: Args) -> Boundary<Args> {
        self::internal::Boundary::upgrade((self.f)(args).await, self).into_update()
    }

    #[cfg(target_arch = "wasm32")]
    pub unsafe fn wasm(
        &'static self,
        event: *const u8,
        event_len: usize,
        out: *mut *const u8,
    ) -> usize
    where
        Args: Clone + Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
    {
        use serde_json::value::RawValue;

        use crate::scope::Scope;

        let event = unsafe { core::slice::from_raw_parts(event, event_len) };
        let event = match core::str::from_utf8(event) {
            Ok(event) => event,
            Err(err) => {
                crate::wasm_exports::fail(format!("failed to parse event as utf8: {err}"));
                return 0;
            }
        };

        #[derive(::serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct JsonEvent {
            event_id: String,
            state: Option<Box<RawValue>>,
            payload: Box<RawValue>,
        }

        let mut event = match serde_json::from_str::<JsonEvent>(event) {
            Ok(event) => event,
            Err(err) => {
                crate::wasm_exports::fail(format!(
                    "failed to parse event as json: {err} (json: `{event}`)"
                ));
                return 0;
            }
        };
        let Some(state_json) = event.state.take() else {
            crate::wasm_exports::fail("missing state in event");
            return 0;
        };
        let args = match serde_json::from_str::<Args>(state_json.get()) {
            Ok(event) => event,
            Err(err) => {
                crate::wasm_exports::fail(format!("failed to parse state as json: {err}"));
                return 0;
            }
        };

        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let scope =
            Scope::new().with_event(event.event_id, crate::scope::Payload::Json(event.payload));
        let result = runtime.block_on(scope.run(async move {
            let r = Renderer::new_update();
            crate::view::FutureExt::into_view(self.with(args))
                .render(r, true)
                .await
        }));
        let crate::render::Out { html, headers } = match result {
            Ok(result) => result.end(),
            Err(err) => {
                crate::wasm_exports::fail(format!("failed to render boundary: {err}"));
                return 0;
            }
        };

        if !headers.is_empty() {
            crate::wasm_exports::fail(format!(
                "headers are unsupported in wasm components, received headers: {}",
                headers
                    .keys()
                    .map(|k| k.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            return 0;
        }

        let len = html.len();
        let html = Box::into_raw(html.into_boxed_str());
        unsafe {
            *out = html as *const u8;
        }
        len
    }
}

pub struct Boundary<Args>
where
    Args: Send + 'static,
{
    boundary_ref: Option<&'static BoundaryRef<Args>>,
    // TODO: take reference to args to avoid cloning them?
    args: Option<Args>,
    view: BoxedView,
    is_update: bool,
}

impl<Args> Boundary<Args>
where
    Args: Send + 'static,
{
    pub(crate) fn new(view: impl View, args: Args) -> Self {
        Boundary {
            boundary_ref: None,
            args: Some(args),
            view: view.boxed(),
            is_update: false,
        }
    }
}

pub mod internal {
    pub use super::*;

    pub trait Boundary<Args>
    where
        Args: Send + 'static,
    {
        fn upgrade(self, id: &'static BoundaryRef<Args>) -> Self;
    }

    impl<Args> Boundary<Args> for super::Boundary<Args>
    where
        Args: Send + 'static,
    {
        fn upgrade(mut self, boundary_ref: &'static BoundaryRef<Args>) -> Self {
            self.boundary_ref = Some(boundary_ref);
            self
        }
    }

    impl<Args, E> Boundary<Args> for Result<super::Boundary<Args>, E>
    where
        Args: Send + 'static,
    {
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
    Args: Send + 'static,
{
    fn into_update(mut self) -> Self {
        self.is_update = true;
        self
    }
}

impl<Args> View for Boundary<Args>
where
    Args: Clone + Serialize + Send + Sync + 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        let Some(args) = self.args else {
            return self.view.render(r, include_hash);
        };

        // TODO: any way to make this a compile error?
        let Some(boundary_ref) = self.boundary_ref else {
            return RenderFuture::Ready(Err(InternalError::MissingBoundaryAttribute.into()));
        };

        let state = match serde_json::to_string(&args) {
            Ok(state) => state,
            Err(err) => {
                return RenderFuture::Ready(Err(InternalError::Serialize {
                    what: "boundary state".into(),
                    err,
                }
                .into()));
            }
        };

        let body = (script(state).r#type("application/json"), self.view);
        if self.is_update {
            body.render(r, include_hash)
        } else {
            Html::<(), _, _>::new("cabin-boundary", boundary_ref, body).render(r, include_hash)
        }
    }
}

impl<Args> Attributes for &'static BoundaryRef<Args>
where
    Args: Send + Sync + 'static,
{
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        r.attribute("name", self.id).map_err(InternalError::from)?;
        r.attribute("events", EventsList(self.events))
            .map_err(InternalError::from)?;
        Ok(())
    }
}

#[derive(Hash)]
struct EventsList(&'static [&'static str]);

impl fmt::Display for EventsList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, ev) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            write!(f, "{ev}")?;
        }

        Ok(())
    }
}

impl<Args, E> From<Result<Boundary<Args>, E>> for Boundary<Args>
where
    Args: Clone + Serialize + Send + Sync + 'static,
    Box<dyn HttpError + Send + 'static>: From<E>,
    E: IntoView + Send + 'static,
{
    fn from(result: Result<Boundary<Args>, E>) -> Self {
        match result {
            Ok(b) => b,
            Err(err) => Boundary {
                boundary_ref: None,
                args: None,
                view: View::boxed(Err::<Boundary<Args>, _>(err)),
                is_update: false,
            },
        }
    }
}
