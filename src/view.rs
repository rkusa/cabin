mod iter;

use std::borrow::Cow;
use std::fmt;

use paste::paste;

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

    fn boxed(self) -> Box<dyn View<M>>
    where
        Self: Sized,
        V: 'static,
    {
        Box::new(self.into_view())
    }
}

// Implementation note: View must be kept object-safe to allow a simple boxed version
// (`Box<dyn View>`).
pub trait View<M = ()> {
    fn render(&self, r: &mut Renderer) -> fmt::Result;
}

impl<M> View<M> for () {
    fn render(&self, _r: &mut Renderer) -> fmt::Result {
        Ok(())
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
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        // TODO: safe escape HTML
        r.text(self)
    }
}

impl<'a, M> IntoView<Cow<'a, str>, M> for Cow<'a, str> {
    fn into_view(self) -> Cow<'a, str> {
        self
    }
}
impl<'a, M> View<M> for Cow<'a, str> {
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        <&str as View<M>>::render(&self.as_ref(), r)
    }
}

impl<M> IntoView<String, M> for String {
    fn into_view(self) -> String {
        self
    }
}
impl<M> View<M> for String {
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        <&str as View<M>>::render(&self.as_str(), r)
    }
}

impl<M> IntoView<Box<dyn View<M>>, M> for Box<dyn View<M>> {
    fn into_view(self) -> Box<dyn View<M>> {
        self
    }
}
impl<M> View<M> for Box<dyn View<M>> {
    fn render(&self, out: &mut Renderer) -> fmt::Result {
        (**self).render(out)
    }
}

impl<V, M> View<M> for Option<V>
where
    V: View<M>,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        match self {
            Some(i) => i.render(r),
            None => Ok(()),
        }
    }
}

impl<V, M> IntoView<Option<V>, M> for Option<V>
where
    V: View<M>,
{
    fn into_view(self) -> Option<V> {
        self
    }
}

impl<'a, V, M> IntoView<&'a V, M> for &'a V
where
    V: View<M>,
{
    fn into_view(self) -> &'a V {
        self
    }
}

impl<'a, V, M> View<M> for &'a V
where
    V: View<M>,
{
    fn render(&self, r: &mut Renderer) -> fmt::Result {
        (*self).render(r)
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $ix:tt ),* ) => {
        paste!{
            impl<$( [<I$ix>]: IntoView<[<V$ix>], M>, [<V$ix>]: View<M> ),*, M> IntoView<($([<V$ix>], )*), M> for ($([<I$ix>],)*) {
                fn into_view(self) -> ($([<V$ix>], )*) {
                    (
                        $(
                            self.$ix.into_view(),
                        )*
                    )
                }
            }

            impl<$( [<V$ix>]: View<M> ),*, M> View<M> for ($( [<V$ix>], )*) {
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
