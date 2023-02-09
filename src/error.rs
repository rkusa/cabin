use std::convert::Infallible;
use std::{error, fmt};

use bytes::Bytes;
use http::{Response, StatusCode};

#[derive(Debug)]
pub struct Error {
    status: StatusCode,
    source: Option<Box<dyn error::Error + Send + Sync + 'static>>,
}

impl Error {
    pub fn from_status_code(status: StatusCode) -> Self {
        Self {
            status,
            source: None,
        }
    }

    pub fn from_err<E: error::Error + Send + Sync + 'static>(err: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            source: Some(Box::new(err)),
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
}

#[derive(Debug)]
pub enum InternalError {
    Render,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|err| err.as_ref() as _)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(err) = &self.source {
            err.fmt(f)
        } else {
            write!(f, "failed with status code {}", self.status)
        }
    }
}

impl error::Error for InternalError {}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InternalError::Render => {
                f.write_str("internal error while rendering component to string")
            }
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
            status: StatusCode::INTERNAL_SERVER_ERROR,
            source: Some(Box::new(err)),
        }
    }
}

impl From<fmt::Error> for InternalError {
    fn from(_: fmt::Error) -> Self {
        InternalError::Render
    }
}

impl From<Error> for Response<Bytes> {
    fn from(err: Error) -> Self {
        Response::builder()
            .status(err.status)
            .body(Bytes::default())
            .unwrap()
    }
}
