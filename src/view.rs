mod any;
pub mod boundary;
mod boxed;
pub mod error;
mod future;
mod iter;
mod macros;
pub mod text;
mod update;

use std::borrow::Cow;
use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use any::AnyView;
pub use boundary::Boundary;
pub use boxed::BoxedView;
pub use future::FutureExt;
use http_error::HttpError;
pub use iter::IteratorExt;
pub use macros::view;
pub use update::UpdateView;

pub use crate::pair::Pair;
use crate::render::{Escape, Renderer};
use crate::view::error::ErrorView;

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View
where
    Self: Send + 'static,
{
    fn render(self, r: Renderer) -> RenderFuture;

    fn boxed(self) -> BoxedView
    where
        Self: Sized + 'static,
    {
        BoxedView::new(self)
    }

    fn boundary<Args: Send>(self, args: Args) -> Boundary<Args>
    where
        Self: Sized,
    {
        Boundary::new(self, args)
    }

    fn into_any_view(self) -> AnyView
    where
        Self: Sized,
    {
        AnyView::new(self)
    }
}

pub enum RenderFuture {
    Ready(Result<Renderer, crate::Error>),
    Future(Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>),
}

impl View for () {
    fn render(self, r: Renderer) -> RenderFuture {
        RenderFuture::Ready(Ok(r))
    }
}

impl View for &'static str {
    fn render(self, r: Renderer) -> RenderFuture {
        Cow::Borrowed(self).render(r)
    }
}

impl View for Cow<'static, str> {
    fn render(self, r: Renderer) -> RenderFuture {
        let mut txt = r.text();
        RenderFuture::Ready(
            Escape::content(&mut txt)
                .write_str(&self)
                .map_err(crate::error::InternalError::from)
                .map_err(crate::error::Error::from)
                .and_then(|_| txt.end()),
        )
    }
}

impl View for String {
    fn render(self, r: Renderer) -> RenderFuture {
        Cow::<'static, str>::Owned(self).render(r)
    }
}

impl<V> View for Option<V>
where
    V: View,
{
    fn render(self, r: Renderer) -> RenderFuture {
        match self {
            Some(i) => i.render(r),
            None => RenderFuture::Ready(Ok(r)),
        }
    }
}

impl<V, E> View for Result<V, E>
where
    V: View,
    Box<dyn HttpError + Send + 'static>: From<E>,
    E: ErrorView + Send + 'static,
{
    fn render(self, r: Renderer) -> RenderFuture {
        match self {
            Ok(v) => v.render(r),
            Err(err) => {
                if err.should_render() {
                    err.into_view().render(r)
                } else {
                    RenderFuture::Ready(Err(crate::Error::from(Box::<
                        dyn HttpError + Send + 'static,
                    >::from(err))))
                }
            }
        }
    }
}

impl Future for RenderFuture {
    type Output = Result<Renderer, crate::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self.get_mut() {
            RenderFuture::Ready(ref mut result) => {
                let result = std::mem::replace(
                    result,
                    Err(crate::error::InternalError::FutureCompleted.into()),
                );
                Poll::Ready(result)
            }
            RenderFuture::Future(ref mut future) => future.as_mut().poll(cx),
        }
    }
}
