mod hash;
mod raw;

use std::fmt::{self, Write};
use std::hash::Hasher;

use twox_hash::XxHash32;

pub use self::hash::ViewHash;
pub use self::raw::raw;

pub trait View<S = ()> {
    fn render(self, out: impl Write) -> Result<ViewHash, fmt::Error>;
}

impl<A> View<A> for () {
    fn render(self, _out: impl Write) -> Result<ViewHash, fmt::Error> {
        Ok(ViewHash::Leaf(0))
    }
}

impl<'a, A> View<A> for &'a str {
    fn render(self, mut out: impl Write) -> Result<ViewHash, fmt::Error> {
        let mut hasher = XxHash32::default();
        hasher.write(self.as_bytes());
        let hash = hasher.finish() as u32;
        // TODO: safe escape HTML
        out.write_str(self)?;
        Ok(ViewHash::Leaf(hash))
    }
}

impl<A> View<A> for String {
    fn render(self, out: impl Write) -> Result<ViewHash, fmt::Error> {
        View::<A>::render(self.as_str(), out)
    }
}

macro_rules! impl_tuple {
    ( $count:tt; $( $t:ident ),+;  $( $ix:tt ),* ) => {
        impl<$( $t: View<A> ),*, A> View<A> for ($( $t, )*) {
            fn render(self, mut out: impl Write) -> Result<ViewHash, fmt::Error> {
                let mut child_hashes = Vec::with_capacity($count);
                $(
                    let hash = self.$ix.render(&mut out)?;
                    child_hashes.push(hash);
                )*
                Ok(ViewHash::Node(0, child_hashes.into_boxed_slice()))
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
