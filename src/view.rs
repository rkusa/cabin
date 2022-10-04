mod boxed;
mod iter;

use std::borrow::Cow;
use std::fmt;
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

impl<M> View<M> for () {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        std::future::ready(Ok(r))
    }
}

impl<M> IntoView<(), M> for () {
    fn into_view(self) {
        self
    }
}

impl<'a, M> IntoView<&'a str, M> for &'a str {
    fn into_view(self) -> &'a str {
        self
    }
}
impl<'a, M> View<M> for &'a str {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, mut r: Renderer) -> Self::Future {
        // TODO: safe escape HTML
        std::future::ready(r.text(self).map(|_| r))
    }
}

impl<'a, M> IntoView<Cow<'a, str>, M> for &'a Cow<'a, str> {
    fn into_view(self) -> Cow<'a, str> {
        Cow::Borrowed(&**self)
    }
}
impl<'a, M> IntoView<Cow<'a, str>, M> for Cow<'a, str> {
    fn into_view(self) -> Cow<'a, str> {
        self
    }
}
impl<'a, M> View<M> for Cow<'a, str> {
    type Future = std::future::Ready<Result<Renderer, fmt::Error>>;

    fn render(self, r: Renderer) -> Self::Future {
        <&str as View<M>>::render(self.as_ref(), r)
    }
}

impl<M> IntoView<String, M> for String {
    fn into_view(self) -> String {
        self
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

impl<V, M> IntoView<Option<V>, M> for Option<V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    V: View<M> + Send + 'static,
{
    fn into_view(self) -> Option<V> {
        self
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $ix:tt ),* ) => {
        paste!{
            // TODO: remove `+ 'static` once removing away from boxed future
            impl<$( [<I$ix>]: IntoView<[<V$ix>], M>, [<V$ix>]: View<M> + Send + 'static),*, M> IntoView<($([<V$ix>], )*), M> for ($([<I$ix>],)*) {
                fn into_view(self) -> ($([<V$ix>], )*) {
                    (
                        $(
                            self.$ix.into_view(),
                        )*
                    )
                }
            }

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
