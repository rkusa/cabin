mod boxed;
mod iter;
pub(crate) mod text;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::future::Future;
use std::pin::Pin;

use paste::paste;

use self::boxed::BoxedView;
use crate::render::Renderer;

// Implementation note: It is not possible to implement [View] for both tupels and iterators
// (fails due to conflicting implementation). As workaround, `IntoView` is introduced as an
// additional indirection. By putting the resulting view into a generic (and not an associated),
// and since both have a different resulting view (the tuple returns itself, and the iterator
// is wrapped into [IteratorView]), it can be implemented for both.
pub trait IntoView<V, M>
where
    V: View<M>,
{
    fn into_view(self) -> V;

    fn boxed(self) -> BoxedView<M>
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
pub trait View<M = ()> {
    type Future: Future<Output = Result<Renderer, fmt::Error>> + Send;
    fn render(self, r: Renderer) -> Self::Future;
}

// This wrapper is necessary to allow the [IntoView] implementation for any [View].
// TODO: better name
pub struct ViewWrapper<V>(V);

impl<V, M> IntoView<ViewWrapper<V>, M> for V
where
    V: View<M>,
{
    fn into_view(self) -> ViewWrapper<V> {
        ViewWrapper(self)
    }
}

impl<V, M> View<M> for ViewWrapper<V>
where
    V: View<M>,
{
    type Future = V::Future;

    fn render(self, r: Renderer) -> Self::Future {
        self.0.render(r)
    }
}

impl<M> View<M> for () {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        std::future::ready(Ok(r))
    }
}

impl<'a, M> View<M> for &'a str {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        // TODO: safe escape HTML
        let mut txt = r.text();
        std::future::ready(txt.write_str(self).and_then(|_| txt.end()))
    }
}

impl<'a, M> IntoView<Cow<'a, str>, M> for &'a Cow<'a, str> {
    fn into_view(self) -> Cow<'a, str> {
        Cow::Borrowed(&**self)
    }
}
impl<'a, M> View<M> for Cow<'a, str> {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        <&str as View<M>>::render(self.as_ref(), r)
    }
}

impl<M> View<M> for String {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        <&str as View<M>>::render(self.as_str(), r)
    }
}

impl<V, M> View<M> for Option<V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    V: View<M> + Send + 'static,
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

macro_rules! impl_tuple {
    ( $count:tt; $( $ix:tt ),* ) => {
        paste!{
            // TODO: remove `+ 'static` once removing away from boxed future
            impl<$( [<V$ix>]: View<M> + Send + 'static),*, M> View<M> for ($( [<V$ix>], )*) {
                // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
                type Future = Pin<Box<dyn Future<Output = Result<Renderer, fmt::Error>> + Send>>;

                fn render(self, r: Renderer) -> Self::Future {
                    Box::pin(async {
                        $(
                            let r = self.$ix.render(r).await?;
                        )*
                        Ok(r)
                    })
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
