pub mod boundary;
mod boxed;
pub mod chunk;
mod future;
mod iter;
pub mod text;

use std::borrow::Cow;
use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;
use std::task;

pub use boundary::Boundary;
pub use future::FutureExt;
use http_error::{AnyHttpError, HttpError};
pub use iter::IteratorExt;

use crate::context::Context;
use crate::render::{Escape, Renderer};
use crate::view::boxed::BoxedView;

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View<'v>: BoxedView<'v>
where
    Self: 'v,
{
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v>;

    fn boxed(self) -> Box<dyn View<'v>>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn boundary<Args>(self, args: Args) -> Boundary<'v, Args>
    where
        Self: Sized,
    {
        Boundary::new(self, args)
    }
}

pub trait IntoView<'v> {
    type View: View<'v>;

    fn into_view(self) -> Self::View;
    fn should_render(&self) -> bool {
        true
    }
}

impl<'v, V> IntoView<'v> for V
where
    V: View<'v>,
{
    type View = V;

    fn into_view(self) -> Self::View {
        self
    }

    fn should_render(&self) -> bool {
        true
    }
}

pub enum RenderFuture<'v> {
    Ready(Option<Result<Renderer, crate::Error>>),
    Future(Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + 'v>>),
}

impl<'v> RenderFuture<'v> {
    pub fn ready(result: Result<Renderer, crate::Error>) -> Self {
        Self::Ready(Some(result))
    }
}

impl<'v> View<'v> for () {
    fn render(self, _c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        RenderFuture::ready(Ok(r))
    }
}

impl<'v> View<'v> for &'v str {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        Cow::Borrowed(self).render(c, r)
    }
}

impl<'v> View<'v> for Cow<'v, str> {
    fn render(self, _c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        let mut txt = r.text();
        RenderFuture::ready(
            Escape::content(&mut txt)
                .write_str(&self)
                .map_err(crate::error::InternalError::from)
                .map_err(crate::error::Error::from)
                .and_then(|_| Ok(txt.end())),
        )
    }
}

impl<'v> View<'v> for String {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        Cow::<'static, str>::Owned(self).render(c, r)
    }
}

impl<'v> View<'v> for &'v String {
    fn render(self, _c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        let mut txt = r.text();
        RenderFuture::ready(
            Escape::content(&mut txt)
                .write_str(&self)
                .map_err(crate::error::InternalError::from)
                .map_err(crate::error::Error::from)
                .and_then(|_| Ok(txt.end())),
        )
    }
}

impl<'v, V> View<'v> for Option<V>
where
    V: View<'v>,
{
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        match self {
            Some(i) => i.render(c, r),
            None => RenderFuture::ready(Ok(r)),
        }
    }
}

impl<'v, V, E> View<'v> for Result<V, E>
where
    V: View<'v>,
    Box<dyn HttpError + Send + 'static>: From<E>,
    E: IntoView<'v> + 'v,
{
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        match self {
            Ok(v) => v.render(c, r),
            Err(err) => {
                if err.should_render() {
                    err.into_view().render(c, r)
                } else {
                    RenderFuture::ready(Err(crate::Error::from(Box::<
                        dyn HttpError + Send + 'static,
                    >::from(err))))
                }
            }
        }
    }
}

impl<'v> IntoView<'v> for AnyHttpError {
    type View = ();

    fn into_view(self) -> Self::View {
        ()
    }

    fn should_render(&self) -> bool {
        false
    }
}

impl<'v> Future for RenderFuture<'v> {
    type Output = Result<Renderer, crate::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        match *self.get_mut() {
            RenderFuture::Ready(ref mut result) => result
                .take()
                .map(task::Poll::Ready)
                .unwrap_or(task::Poll::Pending),
            RenderFuture::Future(ref mut future) => future.as_mut().poll(cx),
        }
    }
}
