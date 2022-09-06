pub(crate) mod hash;

use std::fmt::{self, Write};
use std::hash::Hasher;

pub use self::hash::HashTree;

pub trait View<S = ()> {
    fn render(self, hash_tree: &mut HashTree, out: impl Write) -> fmt::Result;
}

impl<A> View<A> for () {
    fn render(self, _hash_tree: &mut HashTree, _out: impl Write) -> fmt::Result {
        Ok(())
    }
}

impl<'a, A> View<A> for &'a str {
    fn render(self, hash_tree: &mut HashTree, mut out: impl Write) -> fmt::Result {
        let mut node = hash_tree.node();
        node.write(self.as_bytes());
        let hash = node.end();
        // TODO: safe escape HTML
        write!(out, "<!--hash={}-->{}", hash, self)
    }
}

impl<A> View<A> for String {
    fn render(self, hash_tree: &mut HashTree, out: impl Write) -> fmt::Result {
        View::<A>::render(self.as_str(), hash_tree, out)
    }
}

impl<F, V, A> View<A> for F
where
    F: FnOnce() -> V,
    V: View<A>,
{
    fn render(self, hash_tree: &mut HashTree, out: impl Write) -> fmt::Result {
        self().render(hash_tree, out)
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $t:ident ),+;  $( $ix:tt ),* ) => {
        impl<$( $t: View<A> ),*, A> View<A> for ($( $t, )*) {
            fn render(self, hash_tree: &mut HashTree, mut out: impl Write) -> fmt::Result {
                $(
                    self.$ix.render(hash_tree, &mut out)?;
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
