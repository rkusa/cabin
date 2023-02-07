mod boxed;
mod future;
mod iter;
pub(crate) mod text;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::future::Future;
use std::pin::Pin;

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
    type Future: Future<Output = Result<Renderer, fmt::Error>> + Send;
    fn render(self, r: Renderer) -> Self::Future;
}

// This wrapper is necessary to allow the [IntoView] implementation for any [View].
// TODO: better name
pub struct ViewWrapper<V>(V);

impl<V> IntoView<ViewWrapper<V>> for V
where
    V: View,
{
    fn into_view(self) -> ViewWrapper<V> {
        ViewWrapper(self)
    }
}

impl<V> View for ViewWrapper<V>
where
    V: View,
{
    type Future = V::Future;

    fn render(self, r: Renderer) -> Self::Future {
        self.0.render(r)
    }
}

impl View for () {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        std::future::ready(Ok(r))
    }
}

// TODO: escape html!
impl<'a> View for &'a str {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        // TODO: safe escape HTML
        let mut txt = r.text();
        std::future::ready(txt.write_str(self).and_then(|_| txt.end()))
    }
}

impl<'a> IntoView<Cow<'a, str>> for &'a Cow<'a, str> {
    fn into_view(self) -> Cow<'a, str> {
        Cow::Borrowed(&**self)
    }
}
impl<'a> View for Cow<'a, str> {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        <&str as View>::render(self.as_ref(), r)
    }
}

impl View for String {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        <&str as View>::render(self.as_str(), r)
    }
}

impl<V> View for Option<V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    V: View + Send + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

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
        L: View + Send + 'static,
        R: View + Send + 'static,
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
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async {
            let r = self.left.into_view().render(r).await?;
            let r = self.right.into_view().render(r).await?;
            Ok(r)
        })
    }
}

#[macro_export]
macro_rules! view {
    () => (
        ()
    );
    ($left:expr) => (
        $left
    );
    ($left:expr, $right:expr) => (
        $crate::view::Pair::new(
            $crate::view::IntoView::into_view($left),
            $crate::view::IntoView::into_view($right)
        )
    );
    ($left:expr, $($tail:expr),*) => (
        $crate::view::Pair::new(
            $crate::view::IntoView::into_view($left),
            view![$($tail),*]
        )
    );
    ($($x:expr,)*) => (view![$($x),*])
}
