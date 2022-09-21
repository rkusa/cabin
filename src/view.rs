mod iter;
pub mod text;

use std::borrow::Cow;
use std::fmt;

use crate::render::Renderer;

pub use self::text::Text;

use paste::paste;

// Implementation note: It is not possible to implement [View] for both tupels and iterators
// (fails due to conflicting implementation). As workaround, `IntoView` is introduced as an
// additional indirection. By putting the resulting view into a generic (and not an associated),
// and since both have a different resulting view (the tuple returns itself, and the iterator
// is wrapped into [IteratorView]), it can be implemented for both.
pub trait IntoView<V, S>
where
    V: View<S>,
{
    fn into_view(self) -> V;

    fn boxed(self) -> Box<dyn View<S>>
    where
        Self: Sized,
        V: 'static,
    {
        Box::new(self.into_view())
    }
}

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View<S = ()> {
    fn render(&self, r: &mut Renderer) -> fmt::Result;
}

impl<S> View<S> for () {
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        Ok(())
    }
}

impl<S> IntoView<(), S> for () {
    fn into_view(self) {
        self
    }
}

impl<'a, S> View<S> for Cow<'a, str> {
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        // TODO: safe escape HTML
        r.text(&self)
    }
}

impl<'a, S> IntoView<Cow<'a, str>, S> for &'a str {
    fn into_view(self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}

impl<'a, S> IntoView<Cow<'a, str>, S> for Cow<'a, str> {
    fn into_view(self) -> Cow<'a, str> {
        self
    }
}

impl<'a, S> IntoView<Cow<'static, str>, S> for String {
    fn into_view(self) -> Cow<'static, str> {
        Cow::Owned(self)
    }
}

impl<S> IntoView<Box<dyn View<S>>, S> for Box<dyn View<S>> {
    fn into_view(self) -> Box<dyn View<S>> {
        self
    }
}
impl<S> View<S> for Box<dyn View<S>> {
    fn render(&self, out: &mut Renderer) -> fmt::Result {
        (**self).render(out)
    }
}

impl<V, S> View<S> for Option<V>
where
    V: View<S>,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        match self {
            Some(i) => i.render(r),
            None => Ok(()),
        }
    }
}

impl<V, S> IntoView<Option<V>, S> for Option<V>
where
    V: View<S>,
{
    fn into_view(self) -> Option<V> {
        self
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $ix:tt ),* ) => {
        paste!{
            impl<$( [<I$ix>]: IntoView<[<V$ix>], S>, [<V$ix>]: View<S> ),*, S> IntoView<($([<V$ix>], )*), S> for ($([<I$ix>],)*) {
                fn into_view(self) -> ($([<V$ix>], )*) {
                    (
                        $(
                            self.$ix.into_view(),
                        )*
                    )
                }
            }

            impl<$( [<V$ix>]: View<S> ),*, S> View<S> for ($( [<V$ix>], )*) {
                fn render(&self, r: &mut Renderer) -> fmt::Result {
                    $(
                        self.$ix.render(r)?;
                    )*
                    Ok(())
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
