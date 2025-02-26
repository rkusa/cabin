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
        match HeaderValue::from_str(&format!("/client_redirect?{}", self.0)) {
            Ok(location) => Some(vec![(header::LOCATION, location)]),
            Err(err) => {
                tracing::error!(%err, "invalid location header value for empty page error redirect");
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
