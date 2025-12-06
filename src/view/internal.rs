use std::pin::Pin;
use std::task::{self, Poll};

use crate::view::RenderFuture;
use crate::view::chunk::ViewChunk;

pub struct Internal<'v, T>(State<'v, T>);

enum State<'v, T> {
    Builder(T),
    Error(crate::Error),
    Intermediate,
    Render(RenderFuture<'v>),
}

impl<'v, T: Render<'v>> Internal<'v, T> {
    pub fn new(builder: T) -> Self {
        Self(State::Builder(builder))
    }

    pub fn error(error: crate::Error) -> Self {
        Self(State::Error(error))
    }

    pub fn builder_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        match &mut self.0 {
            State::Builder(builder) => Some(builder),
            State::Error(_) | State::Intermediate | State::Render(_) => None,
        }
    }

    pub fn take_builder(self) -> Result<T, crate::Error> {
        match self.0 {
            State::Builder(builder) => Ok(builder),
            State::Error(err) => Err(err),
            State::Intermediate | State::Render(_) => unreachable!(),
        }
    }

    pub fn render(self) -> RenderFuture<'v> {
        let builder = match self.0 {
            State::Builder(builder) => builder,
            State::Error(err) => return RenderFuture::ready(Err(err)),
            State::Intermediate | State::Render(_) => unreachable!(),
        };
        builder.render()
    }

    pub fn errored(&mut self, error: crate::Error) {
        self.0 = State::Error(error);
    }
}

pub trait Render<'v> {
    fn render(self) -> RenderFuture<'v>;
}

impl<'v, T: Render<'v> + Unpin> Future for Internal<'v, T> {
    type Output = ViewChunk;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let state = std::mem::replace(&mut this.0, State::Intermediate);
        let fut = match state {
            State::Builder(builder) => builder.render(),
            State::Error(err) => return Poll::Ready(ViewChunk { result: Err(err) }),
            State::Intermediate => unreachable!("future polled after completion"),
            State::Render(fut) => fut,
        };

        match fut {
            RenderFuture::Ready(mut result) => result
                .take()
                .map(|result| Poll::Ready(ViewChunk { result }))
                .unwrap_or(Poll::Pending),
            RenderFuture::Future(mut future) => match future.as_mut().poll(cx) {
                Poll::Ready(result) => Poll::Ready(ViewChunk { result }),
                Poll::Pending => {
                    this.0 = State::Render(RenderFuture::Future(future));
                    Poll::Pending
                }
            },
        }
    }
}
