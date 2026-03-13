use std::{error, fmt};

use http::{HeaderName, HeaderValue, StatusCode};
use http_error::HttpError;

use crate::View;
use crate::error::InternalError;
use crate::event::Event;

#[derive(Debug)]
pub struct FireEvent {
    event_id: HeaderValue,
    payload: HeaderValue,
}

impl FireEvent {
    pub fn new<E>(event: E) -> Result<Self, cabin::Error>
    where
        E: serde::Serialize + Event + Send + 'static,
    {
        Ok(Self {
            event_id: HeaderValue::from_static(E::ID),
            payload: HeaderValue::from_str(&serde_json::to_string(&event).map_err(|err| {
                InternalError::Serialize {
                    what: "fire event".into(),
                    err,
                }
            })?)
            .map_err(InternalError::InvalidHeaderValue)?,
        })
    }
}

impl View for FireEvent {
    fn render(self, mut r: crate::render::Renderer) -> crate::view::RenderFuture {
        let headers = r.headers_mut();
        headers.insert(
            HeaderName::from_static("cabin-event"),
            self.event_id.clone(),
        );
        headers.insert(
            HeaderName::from_static("cabin-event-payload"),
            self.payload.clone(),
        );
        crate::view::RenderFuture::Ready(Ok(r))
    }
}

impl HttpError for FireEvent {
    fn status_code(&self) -> StatusCode {
        StatusCode::NO_CONTENT
    }

    fn headers(&self) -> Option<Vec<(HeaderName, HeaderValue)>> {
        Some(vec![
            (
                HeaderName::from_static("cabin-event"),
                self.event_id.clone(),
            ),
            (
                HeaderName::from_static("cabin-event-payload"),
                self.payload.clone(),
            ),
        ])
    }
}

impl error::Error for FireEvent {}

impl fmt::Display for FireEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("fire event")
    }
}

impl From<FireEvent> for Box<dyn HttpError + Send + 'static> {
    fn from(err: FireEvent) -> Self {
        Box::new(err)
    }
}
