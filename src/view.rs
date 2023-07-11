mod boxed;
mod future;
mod iter;
pub mod text;

use std::borrow::Cow;
use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use boxed::BoxedView;
// pub use boxed::BoxedView;
pub use future::FutureExt;
use http_error::HttpError;
pub use iter::IteratorExt;
use paste::paste;

use crate::render::{Escape, Renderer};

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View
where
    Self: 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture;

    fn prime(&mut self) {}

    fn boxed(self) -> BoxedView
    where
        Self: Sized + 'static,
    {
        BoxedView::new(self)
    }
}

pub enum RenderFuture {
    Ready(Option<Result<Renderer, crate::Error>>),
    Future(Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>),
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

    fn prime(&mut self) {
        if let Some(inner) = self {
            inner.prime();
        }
    }
}

impl<V, E> View for Result<V, E>
where
    V: View,
    E: HttpError + Send + 'static,
{
    fn render(self, r: Renderer, include_hash: bool) -> RenderFuture {
        match self {
            Ok(v) => v.render(r, include_hash),
            Err(err) => RenderFuture::ready(Err(crate::Error::from_http_err(err))),
        }
    }

    fn prime(&mut self) {
        if let Ok(inner) = self {
            inner.prime();
        }
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
    ( $count:tt; $( $ix:tt ),* ) => {
        paste!{
            impl<$( [<V$ix>]: View + 'static),*> View for ($([<V$ix>],)*) {
                fn render(mut self, r: Renderer, _include_hash: bool) -> RenderFuture {
                    RenderFuture::Future(Box::pin(async move {
                        $(
                            self.$ix.prime();
                        )*
                        $(
                            let r = self.$ix.render(r, true).await?;
                        )*
                        Ok(r)
                    }))
                }
            }
        }
    };
}

impl_tuple!( 1; 0);
impl_tuple!( 2; 0, 1);
impl_tuple!( 3; 0, 1, 2);
impl_tuple!( 4; 0, 1, 2, 3);
impl_tuple!( 5; 0, 1, 2, 3, 4);
impl_tuple!( 6; 0, 1, 2, 3, 4, 5);
impl_tuple!( 7; 0, 1, 2, 3, 4, 5, 6);
impl_tuple!( 8; 0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!( 9; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
