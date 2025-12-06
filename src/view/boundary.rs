use std::fmt;
use std::future::Future;
use std::hash::Hash;
use std::marker::PhantomData;
use std::pin::Pin;

use http_error::HttpError;
use serde::Serialize;

use super::RenderFuture;
use crate::View;
use crate::attribute::{Attribute, WithAttribute as _};
use crate::context::Context;
use crate::element::Element;
use crate::error::InternalError;
use crate::html::elements::script::Script as _;
use crate::render::Renderer;
use crate::view::IntoView;

pub struct Boundary<'v, Args: 'static> {
    boundary_ref: Option<&'static BoundaryRef<Args>>,
    // TODO: take reference to args to avoid cloning them?
    args: Option<Args>,
    view: Box<dyn View<'v>>,
    is_update: bool,
}

impl<'v, Args> Boundary<'v, Args> {
    pub(crate) fn new(view: impl View<'v>, args: Args) -> Self {
        Boundary {
            boundary_ref: None,
            args: Some(args),
            view: view.boxed(),
            is_update: false,
        }
    }
}

type BoundaryFn<Args> = dyn Send
    + Sync
    + for<'v> Fn(&'v Context, Args) -> Pin<Box<dyn Future<Output = Boundary<'v, Args>> + 'v>>;

pub struct BoundaryRef<Args: 'static> {
    pub id: &'static str,
    events: &'static [&'static str],
    f: &'static BoundaryFn<Args>,
}

#[derive(Default)]
pub struct BoundaryEvent<E> {
    marker: PhantomData<E>,
}

impl<Args: 'static> BoundaryRef<Args> {
    pub const fn new(
        id: &'static str,
        events: &'static [&'static str],
        f: &'static BoundaryFn<Args>,
    ) -> Self {
        Self { id, events, f }
    }

    pub async fn with<'v>(&'static self, context: &'v Context, args: Args) -> Boundary<'v, Args> {
        self::internal::Boundary::upgrade((self.f)(context, args).await, self).into_update()
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
        let context = Context::new(true)
            .with_event(event.event_id, crate::context::Payload::Json(event.payload));
        let result = runtime.block_on(async move {
            crate::view::FutureExt::into_view(self.with(&context, args))
                .render(context.acquire_renderer())
                .await
        });
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

pub mod internal {
    pub use super::*;

    pub trait Boundary<Args> {
        fn upgrade(self, id: &'static BoundaryRef<Args>) -> Self;
    }

    impl<'v, Args> Boundary<Args> for super::Boundary<'v, Args> {
        fn upgrade(mut self, boundary_ref: &'static BoundaryRef<Args>) -> Self {
            self.boundary_ref = Some(boundary_ref);
            self
        }
    }

    impl<'v, Args, E> Boundary<Args> for Result<super::Boundary<'v, Args>, E> {
        fn upgrade(mut self, boundary_ref: &'static BoundaryRef<Args>) -> Self {
            if let Ok(b) = &mut self {
                b.boundary_ref = Some(boundary_ref);
            }
            self
        }
    }
}

impl<'v, Args> Boundary<'v, Args> {
    fn into_update(mut self) -> Self {
        self.is_update = true;
        self
    }
}

impl<'v, Args> View<'v> for Boundary<'v, Args>
where
    Args: Clone + Serialize,
{
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        let Some(args) = self.args else {
            return self.view.render(c, r);
        };

        // TODO: any way to make this a compile error?
        let Some(boundary_ref) = self.boundary_ref else {
            return RenderFuture::Ready(Some(Err(InternalError::MissingBoundaryAttribute.into())));
        };

        let state = match serde_json::to_string(&args) {
            Ok(state) => state,
            Err(err) => {
                return RenderFuture::Ready(Some(Err(InternalError::Serialize {
                    what: "boundary state".into(),
                    err,
                }
                .into())));
            }
        };

        let body = c
            .fragment()
            .child(c.script().r#type("application/json").child(state))
            .child(self.view);
        if self.is_update {
            body.render(c, r)
        } else {
            Element::<()>::new(c, "cabin-boundary")
                .with_attribute(boundary_ref)
                .child(body)
                .render(c, r)
        }
    }
}

impl<Args: 'static> Attribute for &'static BoundaryRef<Args> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.attribute("name", self.id);
        r.attribute("events", EventsList(self.events));
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

impl<'v, Args, E> From<Result<Boundary<'v, Args>, E>> for Boundary<'v, Args>
where
    Args: Clone + Serialize,
    Box<dyn HttpError + Send + 'static>: From<E>,
    E: IntoView<'v> + 'v,
{
    fn from(result: Result<Boundary<'v, Args>, E>) -> Self {
        match result {
            Ok(b) => b,
            Err(err) => Boundary {
                boundary_ref: None,
                args: None,
                view: View::boxed(Err::<Boundary<'v, Args>, _>(err)),
                is_update: false,
            },
        }
    }
}
