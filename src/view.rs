pub mod boundary;
mod boxed;
mod future;
mod iter;
pub mod text;
mod update;

use std::borrow::Cow;
use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use boundary::Boundary;
pub use boxed::BoxedView;
pub use future::FutureExt;
use futures_util::future::Either;
use http_error::{AnyHttpError, HttpError};
pub use iter::IteratorExt;
pub use update::UpdateView;

use crate::render::{Escape, Renderer};

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View
where
    Self: Send + 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture;

    fn prime(&mut self) -> impl Future<Output = ()> + Send {
        std::future::ready(())
    }

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
}

pub trait IntoView {
    fn into_view(self) -> impl View;
    fn should_render(&self) -> bool {
        true
    }
}

pub enum RenderFuture {
    Ready(Option<Result<Renderer, crate::Error>>),
    Future(Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>),
}

impl RenderFuture {
    pub fn ready(result: Result<Renderer, crate::Error>) -> RenderFuture {
        RenderFuture::Ready(Some(result))
    }
}

impl View for () {
    fn render(self, r: Renderer, _include_hash: bool) -> RenderFuture {
        RenderFuture::ready(Ok(r))
    }
}

impl View for &'static str {
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        Cow::Borrowed(self).render(r, include_hash)
    }
}

impl View for Cow<'static, str> {
    fn render(self, r: Renderer, _include_hash: bool) -> RenderFuture {
        let mut txt = r.text();
        RenderFuture::ready(
            Escape::content(&mut txt)
                .write_str(&self)
                .map_err(crate::error::InternalError::from)
                .map_err(crate::error::Error::from)
                .and_then(|_| txt.end()),
        )
    }
}

impl View for String {
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        Cow::<'static, str>::Owned(self).render(r, include_hash)
    }
}

impl<V> View for Option<V>
where
    V: View,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        match self {
            Some(i) => i.render(r, include_hash),
            None => RenderFuture::ready(Ok(r)),
        }
    }

    fn prime(&mut self) -> impl Future<Output = ()> + Send {
        if let Some(inner) = self {
            Either::Left(inner.prime())
        } else {
            Either::Right(std::future::ready(()))
        }
    }
}

impl<V, E> View for Result<V, E>
where
    V: View,
    Box<dyn HttpError + Send + 'static>: From<E>,
    E: IntoView + Send + 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        match self {
            Ok(v) => v.render(r, include_hash),
            Err(err) => {
                if err.should_render() {
                    err.into_view().render(r, include_hash)
                } else {
                    RenderFuture::ready(Err(crate::Error::from(Box::<
                        dyn HttpError + Send + 'static,
                    >::from(err))))
                }
            }
        }
    }

    async fn prime(&mut self) {
        if let Ok(inner) = self {
            inner.prime().await;
        }
    }
}

impl IntoView for AnyHttpError {
    fn into_view(self) -> impl View {}

    fn should_render(&self) -> bool {
        false
    }
}

impl Future for RenderFuture {
    type Output = Result<Renderer, crate::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match *self.get_mut() {
            RenderFuture::Ready(ref mut result) => {
                result.take().map(Poll::Ready).unwrap_or(Poll::Pending)
            }
            RenderFuture::Future(ref mut future) => future.as_mut().poll(cx),
        }
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $ix:tt ),*; $( $generic:tt ),* ) => {
        impl<$( $generic: View ),*> View for ($($generic,)*) {
            fn render(mut self, r: Renderer, _include_hash: bool) -> RenderFuture {
                RenderFuture::Future(Box::pin(async move {
                    tokio::join!($(
                        self.$ix.prime(),
                    )*);
                    $(
                        let r = self.$ix.render(r, true).await?;
                    )*
                    Ok(r)
                }))
            }
        }
    };
}

impl_tuple!( 1; 0; V0);
impl_tuple!( 2; 0, 1; V0, V1);
impl_tuple!( 3; 0, 1, 2; V0, V1, V2);
impl_tuple!( 4; 0, 1, 2, 3; V0, V1, V2, V3);
impl_tuple!( 5; 0, 1, 2, 3, 4; V0, V1, V2, V3, V4);
impl_tuple!( 6; 0, 1, 2, 3, 4, 5; V0, V1, V2, V3, V4, V5);
impl_tuple!( 7; 0, 1, 2, 3, 4, 5, 6; V0, V1, V2, V3, V4, V5, V6);
impl_tuple!( 8; 0, 1, 2, 3, 4, 5, 6, 7; V0, V1, V2, V3, V4, V5, V6, V7);
impl_tuple!( 9; 0, 1, 2, 3, 4, 5, 6, 7, 8; V0, V1, V2, V3, V4, V5, V6, V7, V8);
impl_tuple!(10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9; V0, V1, V2, V3, V4, V5, V6, V7, V8, V9);
