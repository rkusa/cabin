use http::{HeaderName, HeaderValue};

use crate::error::InternalError;
use crate::render::Renderer;
use crate::view::RenderFuture;
use crate::View;

use super::SerializeEventFn;

pub struct FireEvent(pub Box<SerializeEventFn>);

pub fn fire_event<E>(event: E) -> FireEvent
where
    E: serde::Serialize + 'static,
{
    FireEvent(Box::new(move || {
        use std::hash::{Hash, Hasher};

        let mut hasher = twox_hash::XxHash32::default();
        std::any::TypeId::of::<E>().hash(&mut hasher);
        let hash = hasher.finish() as u32;
        serde_json::to_string(&event)
            .map_err(|err| InternalError::Serialize {
                what: "fire_event",
                err,
            })
            .map(|json| (hash, json))
    }))
}

impl View for FireEvent {
    fn render(self, mut r: Renderer, _include_hash: bool) -> RenderFuture {
        if r.is_update() {
            let (id, payload) = match (self.0)() {
                Ok(ok) => ok,
                Err(err) => return RenderFuture::Ready(Some(Err(err.into()))),
            };
            let id_header = match HeaderValue::from_str(&id.to_string()) {
                Ok(v) => v,
                Err(err) => {
                    tracing::error!(%err, "invalid header value for X-CABIN-EVENT");
                    return RenderFuture::Ready(Some(Ok(r)));
                }
            };
            let payload_header = match HeaderValue::from_str(&payload) {
                Ok(v) => v,
                Err(err) => {
                    tracing::error!(%err, "invalid header value for X-CABIN-PAYLOAD");
                    return RenderFuture::Ready(Some(Ok(r)));
                }
            };

            let h = r.headers_mut();
            h.insert(HeaderName::from_static("x-cabin-event"), id_header);
            h.insert(HeaderName::from_static("x-cabin-payload"), payload_header);
        }

        RenderFuture::Ready(Some(Ok(r)))
    }
}
