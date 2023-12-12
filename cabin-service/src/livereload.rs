use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use bytes::Bytes;
use http_body::{Body, Frame};
use tokio::time::{interval, Interval};

pub struct Heartbeat {
    interval: Interval,
}

impl Body for Heartbeat {
    type Data = Bytes;
    type Error = Infallible;

    #[inline]
    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match self.as_mut().interval.poll_tick(cx) {
            Poll::Ready(_) => Poll::Ready(Some(Ok(Frame::data(Bytes::from(": heartbeat"))))),
            Poll::Pending => Poll::Pending,
        }
    }

    #[inline]
    fn is_end_stream(&self) -> bool {
        false
    }
}

impl Default for Heartbeat {
    fn default() -> Self {
        Self {
            interval: interval(Duration::from_secs(10)),
        }
    }
}
