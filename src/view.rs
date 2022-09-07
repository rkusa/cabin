pub(crate) mod hash;
mod iter;

use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::Hasher;

pub use self::hash::HashTree;
pub use self::iter::list;

pub trait View<S = ()> {
    type Renderer: Render;
    fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer>;
}

pub trait Render {
    fn render(self, out: impl Write, is_update: bool) -> fmt::Result;
}

impl<S> View<S> for () {
    type Renderer = ();

    fn render(self, _hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        Some(())
    }
}

impl Render for () {
    fn render(self, _out: impl Write, _is_update: bool) -> fmt::Result {
        Ok(())
    }
}

impl<'a, S> View<S> for &'a str {
    type Renderer = Cow<'a, str>;

    fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        let mut node = hash_tree.node();
        node.write(self.as_bytes());
        let hash = node.end();
        hash_tree.changed_or_else(hash, || Cow::Borrowed(self))
    }
}

impl<'a, S> View<S> for Cow<'a, str> {
    type Renderer = Cow<'a, str>;

    fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        let mut node = hash_tree.node();
        node.write(self.as_bytes());
        let hash = node.end();
        hash_tree.changed_or_else(hash, || self)
    }
}

impl<S> View<S> for String {
    type Renderer = Cow<'static, str>;

    fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        let mut node = hash_tree.node();
        node.write(self.as_bytes());
        let hash = node.end();
        hash_tree.changed_or_else(hash, || Cow::Owned(self))
    }
}

impl<'a> Render for Cow<'a, str> {
    fn render(self, mut out: impl Write, _is_update: bool) -> fmt::Result {
        // TODO: safe escape HTML
        out.write_str(&self)
    }
}

impl<F, V, S> View<S> for F
where
    F: FnOnce() -> V,
    V: View<S>,
{
    type Renderer = V::Renderer;

    fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
        self().render(hash_tree)
    }
}

impl<R> Render for Option<R>
where
    R: Render,
{
    fn render(self, mut out: impl Write, is_update: bool) -> fmt::Result {
        match self {
            Some(r) => r.render(out, is_update),
            None => write!(out, "<!--unchanged-->"),
        }
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $t:ident ),+;  $( $ix:tt ),* ) => {
        impl<$( $t: View<S> ),*, S> View<S> for ($( $t, )*) {
            type Renderer = ($( Option<$t::Renderer>, )*);

            fn render(self, hash_tree: &mut HashTree) -> Option<Self::Renderer> {
                Some((
                    $(
                        self.$ix.render(hash_tree),
                    )*
                ))
            }
        }

        impl<$( $t: Render ),*> Render for ($( Option<$t>, )*) {
            fn render(self, mut out: impl Write, is_update: bool) -> fmt::Result {
                $(
                    self.$ix.render(&mut out, is_update)?;
                )*
                Ok(())
            }
        }
    };
}

impl_tuple!( 1; V1; 0);
impl_tuple!( 2; V1, V2; 0, 1);
impl_tuple!( 3; V1, V2, V3; 0, 1, 2);
impl_tuple!( 4; V1, V2, V3, V4; 0, 1, 2, 3);
impl_tuple!( 5; V1, V2, V3, V4, V5; 0, 1, 2, 3, 4);
impl_tuple!( 6; V1, V2, V3, V4, V5, V6; 0, 1, 2, 3, 4, 5);
impl_tuple!( 7; V1, V2, V3, V4, V5, V6, V7; 0, 1, 2, 3, 4, 5, 6);
impl_tuple!( 8; V1, V2, V3, V4, V5, V6, V7, V8; 0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!( 9; V1, V2, V3, V4, V5, V6, V7, V8, V9; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(10; V1, V2, V3, V4, V5, V6, V7, V8, V9, V10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
