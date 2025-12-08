use std::any::Any;
use std::fmt;
use std::future::Future;
use std::hash::Hash;
use std::marker::PhantomData;
use std::pin::Pin;

use serde::Serialize;

use crate::View;
use crate::attribute::{Attribute, WithAttribute as _};
use crate::element::Element;
use crate::error::InternalError;
use crate::render::Renderer;
use crate::view::AnyView;
use crate::view::any::IntoAnyView;

pub struct Boundary<Args: 'static> {
    boundary_ref: Option<&'static BoundaryRef<Args>>,
    // TODO: take reference to args to avoid cloning them?
    args: Option<Args>,
    view: AnyView,
}

impl<Args> Boundary<Args> {
    pub(crate) fn new(view: AnyView, args: Args) -> Self {
        Boundary {
            boundary_ref: None,
            args: Some(args),
            view,
        }
    }
}

impl<Args: Serialize> Boundary<Args> {
    pub fn any(self) -> AnyView {
        crate::h::any(self)
    }
}

type BoundaryFn<Args> = dyn Send + Sync + Fn(Args) -> Pin<Box<dyn Future<Output = AnyView>>>;

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

    pub async fn render(&'static self, args: Args) -> AnyView {
        (self.f)(args).await
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
        let context = crate::Context::new(true)
            .with_event(event.event_id, crate::context::Payload::Json(event.payload));
        let mut r = context.acquire_renderer();
        let result = runtime.block_on(context.run(async move {
            let doc = self.render(args).await;
            doc.render(&mut r)?;
            Ok(r.end())
        }));
        let crate::render::Out { html, headers } = match result {
            Ok(out) => out,
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

impl<Args> View for Boundary<Args>
where
    Args: Serialize,
{
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        use cabin::prelude::*;

        let Some(args) = self.args else {
            return self.view.render(r);
        };

        // TODO: any way to make this a compile error?
        let Some(boundary_ref) = self.boundary_ref else {
            return Err(InternalError::MissingBoundaryAttribute.into());
        };

        let state = match serde_json::to_string(&args) {
            Ok(state) => state,
            Err(err) => {
                return Err(InternalError::Serialize {
                    what: "boundary state".into(),
                    err,
                }
                .into());
            }
        };

        let body = h::fragment()
            .child(h::script().r#type("application/json").child(state))
            .child(self.view);
        if r.is_update() {
            View::render(body, r)
        } else {
            Element::<()>::new("cabin-boundary")
                .with_attribute(boundary_ref)
                .child(body)
                .render(r)
        }
    }
}

impl<Args: Serialize> IntoAnyView for Boundary<Args> {
    fn into_any_view(self) -> AnyView {
        self.any()
    }
}

impl<Args: 'static> Attribute for &'static BoundaryRef<Args> {
    fn render(self, r: &mut Renderer) -> Result<(), crate::Error> {
        r.attribute("name", self.id);
        r.attribute("events", EventsList(self.events));
        Ok(())
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self as &dyn Any)
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
