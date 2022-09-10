pub mod any;
pub(crate) mod hash;
mod iter;
pub mod text;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;

pub use self::any::AnyView;
pub use self::hash::HashTree;
pub use self::text::Text;

use paste::paste;

pub trait IntoView<V, S>
where
    V: View<S>,
{
    fn into_view(self) -> V;
}

pub trait View<S = ()> {
    type Render: Render;
    fn prepare(self, hash_tree: &mut HashTree) -> Option<Self::Render>;
}

pub trait Render {
    fn render(&self, out: &mut dyn Write, is_update: bool) -> fmt::Result;
}

impl<S> View<S> for () {
    type Render = ();

    fn prepare(self, _hash_tree: &mut HashTree) -> Option<Self::Render> {
        Some(())
    }
}

impl<S> IntoView<(), S> for () {
    fn into_view(self) -> () {
        self
    }
}

impl Render for () {
    fn render(&self, _out: &mut dyn Write, _is_update: bool) -> fmt::Result {
        Ok(())
    }
}

impl<'a, S> View<S> for &'a str {
    type Render = Cow<'a, str>;

    fn prepare(self, hash_tree: &mut HashTree) -> Option<Self::Render> {
        let mut node = hash_tree.node();
        node.write(self.as_bytes());
        let hash = node.end();
        hash_tree.changed_or_else(hash, || Cow::Borrowed(self))
    }
}

impl<'a, S> IntoView<&'a str, S> for &'a str {
    fn into_view(self) -> &'a str {
        self
    }
}

impl<'a, S> View<S> for Cow<'a, str> {
    type Render = Cow<'a, str>;

    fn prepare(self, hash_tree: &mut HashTree) -> Option<Self::Render> {
        let mut node = hash_tree.node();
        node.write(self.as_bytes());
        let hash = node.end();
        hash_tree.changed_or_else(hash, || self)
    }
}

impl<'a, S> IntoView<Cow<'a, str>, S> for Cow<'a, str> {
    fn into_view(self) -> Cow<'a, str> {
        self
    }
}

impl<S> View<S> for String {
    type Render = Cow<'static, str>;

    fn prepare(self, hash_tree: &mut HashTree) -> Option<Self::Render> {
        let mut node = hash_tree.node();
        node.write(self.as_bytes());
        let hash = node.end();
        hash_tree.changed_or_else(hash, || Cow::Owned(self))
    }
}

impl<S> IntoView<String, S> for String {
    fn into_view(self) -> String {
        self
    }
}

impl<'a> Render for Cow<'a, str> {
    fn render(&self, out: &mut dyn Write, _is_update: bool) -> fmt::Result {
        // TODO: safe escape HTML
        out.write_str(self)
    }
}

// Disabled as it leads to unhelpful errors (
// > expected an `FnOnce<()>` closure, found `V`
// instead of
// > the trait `View<_>` is not implemented for `V`
//
// impl<F, V, S> View<S> for F
// where
//     F: FnOnce() -> V,
//     V: View<S>,
// {
//     type Renderer = V::Renderer;

//     fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
//         self().render(hash_tree)
//     }
// }

impl<V, S> View<S> for Option<V>
where
    V: View<S>,
{
    type Render = OptionRenderer<V::Render>;

    fn prepare(self, hash_tree: &mut HashTree) -> Option<Self::Render> {
        Some(OptionRenderer(self.and_then(|v| v.prepare(hash_tree))))
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

pub struct OptionRenderer<R>(Option<R>);

impl<R> Render for OptionRenderer<R>
where
    R: Render,
{
    fn render(&self, out: &mut dyn Write, is_update: bool) -> fmt::Result {
        match &self.0 {
            Some(r) => r.render(out, is_update),
            None => Ok(()),
        }
    }
}

impl<R> Render for Option<R>
where
    R: Render,
{
    fn render(&self, out: &mut dyn Write, is_update: bool) -> fmt::Result {
        match self {
            Some(r) => r.render(out, is_update),
            None => write!(out, "<!--unchanged-->"),
        }
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
                type Render = ($( Option<[<V$ix>]::Render>, )*);

                fn prepare(self, hash_tree: &mut HashTree) -> Option<Self::Render> {
                    Some((
                        $(
                            self.$ix.prepare(hash_tree),
                        )*
                    ))
                }
            }

            impl<$( [<V$ix>]: Render ),*> Render for ($( Option<[<V$ix>]>, )*) {
                fn render(&self, mut out: &mut dyn Write, is_update: bool) -> fmt::Result {
                    $(
                        self.$ix.render(&mut out, is_update)?;
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
