// mod boxed;
// mod future;
// mod iter;
pub mod text;

use std::borrow::Cow;
use std::fmt::Write;

// pub use boxed::BoxedView;
// pub use future::FutureExt;
// pub use iter::IteratorExt;
use paste::paste;

use crate::render::Renderer;

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View {
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error>;

    fn prime(&mut self) {}
}

impl View for () {
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        Ok(r)
    }
}

// TODO: escape html!
impl<'a> View for &'a str {
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        // TODO: safe escape HTML
        let mut txt = r.text();
        txt.write_str(self)
            .map_err(crate::error::InternalError::from)
            .map_err(crate::error::Error::from)
            .and_then(|_| txt.end())
    }
}

impl<'a> View for Cow<'a, str> {
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        <&str as View>::render(self.as_ref(), r).await
    }
}

impl View for String {
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        <&str as View>::render(self.as_str(), r).await
    }
}

// impl<V> View for Option<V>
// where
//     V: View,
// {
//     async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
//         match self {
//             Some(i) => i.render(r).await,
//             None => Ok(r),
//         }
//     }

//     fn prime(&mut self) {
//         if let Some(inner) = self {
//             inner.prime();
//         }
//     }
// }

impl<V, E> View for Result<V, E>
where
    V: View,
    crate::Error: From<E>,
{
    async fn render(self, r: Renderer) -> Result<Renderer, crate::Error> {
        match self {
            Ok(v) => v.render(r).await,
            Err(err) => Err(err.into()),
        }
    }

    fn prime(&mut self) {
        if let Ok(inner) = self {
            inner.prime();
        }
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $ix:tt ),* ) => {
        paste!{
            impl<$( [<V$ix>]: View),*> View for ($([<V$ix>],)*) {
                async fn render(mut self, r: Renderer) -> Result<Renderer, crate::Error> {
                    $(
                        self.$ix.prime();
                    )*
                    $(
                        let r = self.$ix.render(r).await?;
                    )*
                    Ok(r)
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
