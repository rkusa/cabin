use std::convert::Infallible;
use std::{error, fmt};

use bytes::Bytes;
use http::{Response, StatusCode};
use http_error::HttpError;

#[derive(Debug)]
pub struct Error {
    inner: Inner,
}

#[derive(Debug)]
enum Inner {
    Http {
        status: Option<StatusCode>,
        source: Box<dyn HttpError + Send + 'static>,
    },
    Other {
        status: StatusCode,
        reason: Option<String>,
        source: Option<Box<dyn error::Error + Send + 'static>>,
    },
}

impl Error {
    pub fn from_status_code(status: StatusCode) -> Self {
        Self {
            inner: Inner::Other {
                status,
                reason: None,
                source: None,
            },
        }
    }

    pub fn from_status_code_and_reason(status: StatusCode, reason: impl Into<String>) -> Self {
        Self {
            inner: Inner::Other {
                status,
                reason: Some(reason.into()),
                source: None,
            },
        }
    }

    pub fn from_err<E: error::Error + Send + 'static>(err: E) -> Self {
        Self {
            inner: Inner::Other {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                reason: None,
                source: Some(Box::new(err)),
            },
        }
    }

    pub fn from_http_err<E: HttpError + Send + 'static>(err: E) -> Self {
        Self {
            inner: Inner::Http {
                status: None,
                source: Box::new(err),
            },
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.inner = match self.inner {
            Inner::Http { source, .. } => Inner::Http {
                status: Some(status),
                source,
            },
            Inner::Other { source, .. } => Inner::Other {
                status,
                reason: None,
                source,
            },
        };
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.inner = match self.inner {
            Inner::Http { status, source } => Inner::Other {
                status: status.unwrap_or_else(|| source.status_code()),
                reason: Some(reason.into()),
                source: Some(Box::new(InnerHttpError(source))),
            },
            Inner::Other { status, source, .. } => Inner::Other {
                status,
                reason: Some(reason.into()),
                source,
            },
        };
        self
    }
}

#[derive(Debug)]
pub enum InternalError {
    Render,
    Serialize {
        what: &'static str,
        err: serde_json::Error,
    },
    Deserialize {
        what: &'static str,
        err: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    InvalidAttributeName {
        name: String,
    },
    Join(tokio::task::JoinError),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.inner {
            Inner::Http { source, .. } => source.source(),
            Inner::Other { source, .. } => source.as_ref().map(|err| err.as_ref() as _),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            Inner::Http { source, .. } => source.fmt(f),
            Inner::Other { status, source, .. } => {
                if let Some(err) = source {
                    err.fmt(f)
                } else {
                    write!(f, "failed with status code {}", status)
                }
            }
        }
    }
}

impl error::Error for InternalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Render | Self::InvalidAttributeName { .. } => None,
            Self::Serialize { err, .. } => Some(err),
            Self::Deserialize { err, .. } => Some(err.as_ref()),
            Self::Join(err) => Some(err),
        }
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Render => f.write_str("internal error while rendering view to string"),
            Self::Serialize { what, .. } => write!(f, "failed to serialize {what}"),
            Self::Deserialize { what, .. } => write!(f, "failed to deserialize {what}"),
            Self::InvalidAttributeName { name } => {
                write!(f, "invalid attribute name `{name}`")
            }
            Self::Join(_) => f.write_str("failed to run internal future to completion"),
        }
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::from_status_code(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<InternalError> for Error {
    fn from(err: InternalError) -> Self {
        Self {
            inner: Inner::Other {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                reason: None,
                source: Some(Box::new(err)),
            },
        }
    }
}

impl From<fmt::Error> for InternalError {
    fn from(_: fmt::Error) -> Self {
        InternalError::Render
    }
}

impl From<tokio::task::JoinError> for InternalError {
    fn from(err: tokio::task::JoinError) -> Self {
        InternalError::Join(err)
    }
}

impl From<Box<dyn HttpError + Send + 'static>> for Error {
    fn from(err: Box<dyn HttpError + Send + 'static>) -> Self {
        Self {
            inner: Inner::Http {
                status: None,
                source: err,
            },
        }
    }
}

impl From<Error> for Box<dyn HttpError + Send + 'static> {
    fn from(err: Error) -> Self {
        match err.inner {
            Inner::Http {
                status: None,
                source,
            } => source,
            _ => Box::new(err),
        }
    }
}

impl HttpError for Error {
    fn status_code(&self) -> StatusCode {
        match &self.inner {
            Inner::Http { status, source } => status.unwrap_or_else(|| source.status_code()),
            Inner::Other { status, .. } => *status,
        }
    }

    fn reason(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Inner::Other {
            reason: Some(reason),
            ..
        } = &self.inner
        {
            return f.write_str(reason);
        }
        if let Some(reason) = self.status_code().canonical_reason() {
            f.write_str(reason)?;
        }
        Ok(())
    }
}

impl From<Error> for Response<Bytes> {
    fn from(err: Error) -> Self {
        match err.inner {
            Inner::Http { status, source } => {
                let mut res = Response::new(Bytes::default());
                *res.status_mut() = status.unwrap_or_else(|| source.status_code());
                if let Some(headers) = source.headers() {
                    let h = res.headers_mut();
                    for (name, value) in headers {
                        h.insert(name, value);
                    }
                }
                res
            }
            Inner::Other { status, .. } => Response::builder()
                .status(status)
                .body(Bytes::default())
                .unwrap(),
        }
    }
}

#[derive(Debug)]
struct InnerHttpError(Box<dyn HttpError + Send + 'static>);

impl fmt::Display for InnerHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for InnerHttpError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.0.source()
    }
}
