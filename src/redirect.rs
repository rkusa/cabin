use std::borrow::Cow;
use std::{error, fmt};

use http::{HeaderName, HeaderValue, StatusCode, header};
use http_error::HttpError;

#[derive(Debug)]
pub struct Redirect(pub Cow<'static, str>);

impl Redirect {
    pub fn new(to: impl Into<Cow<'static, str>>) -> Self {
        Redirect(to.into())
    }
}

impl HttpError for Redirect {
    fn status_code(&self) -> StatusCode {
        StatusCode::SEE_OTHER
    }

    fn headers(&self) -> Option<Vec<(HeaderName, HeaderValue)>> {
        // The `x-cabin-redirect` header marks the response as a cabin redirect, which
        // `cabin_service::redirects` turns into a 204 for requests from cabin.js (so that `fetch`
        // doesn't follow it and load the target).
        match HeaderValue::from_str(&self.0) {
            Ok(location) => Some(vec![
                (header::LOCATION, location.clone()),
                (HeaderName::from_static("x-cabin-redirect"), location),
            ]),
            Err(err) => {
                tracing::error!(%err, "invalid location header value for redirect");
                None
            }
        }
    }
}

impl error::Error for Redirect {}

impl fmt::Display for Redirect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("cabin redirect")
    }
}
