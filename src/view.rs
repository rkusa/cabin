use std::fmt::{self, Write};

pub trait View<A = ()> {
    fn render(self, out: impl Write) -> fmt::Result;
}

impl<A> View<A> for () {
    fn render(self, _out: impl Write) -> fmt::Result {
        Ok(())
    }
}

impl<'a, A> View<A> for &'a str {
    fn render(self, mut out: impl Write) -> fmt::Result {
        // TODO: safe escape HTML
        out.write_str(self)?;
        Ok(())
    }
}

impl<A> View<A> for String {
    fn render(self, out: impl Write) -> fmt::Result {
        View::<A>::render(self.as_str(), out)
    }
}

macro_rules! impl_tuple {
    ( $( $t:ident ),+ ;  $( $ix:tt ),* ) => {
        impl<$( $t: View<A> ),*, A> View<A> for ($( $t, )*) {
            fn render(self, mut out: impl Write) -> fmt::Result {
                $(
                    self.$ix.render(&mut out)?;
                )*
                Ok(())
            }
        }
    };
}

impl_tuple!(V1; 0);
impl_tuple!(V1, V2; 0, 1);
impl_tuple!(V1, V2, V3; 0, 1, 2);
impl_tuple!(V1, V2, V3, V4; 0, 1, 2, 3);
impl_tuple!(V1, V2, V3, V4, V5; 0, 1, 2, 3, 4);
impl_tuple!(V1, V2, V3, V4, V5, V6; 0, 1, 2, 3, 4, 5);
impl_tuple!(V1, V2, V3, V4, V5, V6, V7; 0, 1, 2, 3, 4, 5, 6);
impl_tuple!(V1, V2, V3, V4, V5, V6, V7, V8; 0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(V1, V2, V3, V4, V5, V6, V7, V8, V9, V10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
