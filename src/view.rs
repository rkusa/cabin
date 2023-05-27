mod boxed;
mod future;
mod iter;
pub(crate) mod text;

use std::borrow::Cow;
use std::fmt::Write;
use std::future::Future;
use std::pin::Pin;

pub use future::FutureExt;
pub use iter::IteratorExt;
use paste::paste;

use self::boxed::BoxedView;
use crate::render::Renderer;

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View {
    type Future: Future<Output = Result<Renderer, crate::Error>>;

    fn render(self, r: Renderer) -> Self::Future;

    fn boxed(self) -> BoxedView
    where
        Self: Sized + 'static,
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

impl<V> View for Option<V>
where
    V: View + 'static,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async {
            match self {
                Some(i) => i.render(r).await,
                None => Ok(r),
            }
        })
    }
}

impl<V, E> View for Result<V, E>
where
    V: View + 'static,
    E: 'static,
    crate::Error: From<E>,
{
    // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
    type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

    fn render(self, r: Renderer) -> Self::Future {
        Box::pin(async {
            match self {
                Ok(v) => v.render(r).await,
                Err(err) => Err(err.into()),
            }
        })
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $ix:tt ),* ) => {
        paste!{
            impl<$( [<V$ix>]: View + 'static),*> View for ($([<V$ix>],)*) {
                // TODO: move to `impl Future` once `type_alias_impl_trait` is stable
                type Future = Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>>>>;

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
