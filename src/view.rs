mod boxed;
mod fragment;
mod future;
mod iter;
pub(crate) mod text;

use std::borrow::Cow;
use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;

pub use fragment::*;

use self::boxed::BoxedView;
use crate::render::Renderer;

// Implementation note: It is not possible to implement [View] for both tupels and iterators
// (fails due to conflicting implementation). As workaround, `IntoView` is introduced as an
// additional indirection. By putting the resulting view into a generic (and not an associated),
// and since both have a different resulting view (the tuple returns itself, and the iterator
// is wrapped into [IteratorView]), it can be implemented for both.
pub trait IntoView<V>
where
    V: View,
{
    fn into_view(self) -> V;

    fn boxed(self) -> BoxedView
    where
        Self: Sized,
        V: Send + 'static,
        V::Future: 'static,
    {
        BoxedView::new(self.into_view())
    }
}

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View: Send {
    type Future: Future<Output = Result<Renderer, crate::Error>> + Send;

    fn render(self, r: Renderer) -> Self::Future;

    fn boxed(self) -> BoxedView
    where
        Self: Sized + 'static,
        Self::Future: 'static,
    {
        BoxedView::new(self)
    }
}

impl View for () {
    type Future = std::future::Ready<Result<Renderer, crate::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        std::future::ready(Ok(r))
    }
}

// TODO: escape html!
impl<'a> View for &'a str {
    type Future = std::future::Ready<Result<Renderer, crate::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        // TODO: safe escape HTML
        let mut txt = r.text();
        std::future::ready(
            txt.write_str(self)
                .map_err(crate::error::InternalError::from)
                .map_err(crate::error::Error::from)
                .and_then(|_| txt.end()),
        )
    }
}

impl<'a> View for Cow<'a, str> {
    type Future = std::future::Ready<Result<Renderer, crate::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        <&str as View>::render(self.as_ref(), r)
    }
}

impl View for String {
    type Future = std::future::Ready<Result<Renderer, crate::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        <&str as View>::render(self.as_str(), r)
    }
}

impl<V, E> View for Result<V, E>
where
    V: View + 'static,
    E: Send + 'static,
    crate::Error: From<E>,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async {
            match self {
                Ok(v) => v.render(r).await,
                Err(err) => Err(err.into()),
            }
        })
    }
}

impl<V> View for Option<V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    V: View + Send + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async {
            match self {
                Some(i) => i.render(r).await,
                None => Ok(r),
            }
        })
    }
}

pub struct Pair<L, R> {
    left: L,
    right: R,
}

impl<L, R> Pair<L, R> {
    pub fn new(left: L, right: R) -> Self
    where
        L: View,
        R: View,
    {
        Pair { left, right }
    }
}

impl<L, R> View for Pair<L, R>
where
    L: View + Send + 'static,
    R: View + Send + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async {
            let r = self.left.render(r).await?;
            let r = self.right.render(r).await?;
            Ok(r)
        })
    }
}
