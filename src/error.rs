use std::convert::Infallible;
use std::{error, fmt};

use bytes::Bytes;
use http::{Response, StatusCode};

#[derive(Debug)]
pub struct Error {
    status: StatusCode,
    source: Option<Box<dyn error::Error + Send + 'static>>,
}

impl Error {
    pub fn from_status_code(status: StatusCode) -> Self {
        Self {
            status,
            source: None,
        }
    }

    pub fn from_err<E: error::Error + Send + 'static>(err: E) -> Self {
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
    Serialize {
        what: &'static str,
        err: serde_json::Error,
    },
    Deserialize {
        what: &'static str,
        err: serde_json::Error,
    },
    InvalidAttributeName {
        name: String,
    },
    Join(tokio::task::JoinError),
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

impl error::Error for InternalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Render | Self::InvalidAttributeName { .. } => None,
            Self::Serialize { err, .. } => Some(err),
            Self::Deserialize { err, .. } => Some(err),
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

impl From<tokio::task::JoinError> for InternalError {
    fn from(err: tokio::task::JoinError) -> Self {
        InternalError::Join(err)
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
