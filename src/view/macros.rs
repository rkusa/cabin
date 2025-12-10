#[macro_export]
macro_rules! view {
    () => (
        ()
    );
    ($head:expr $(,)?) => (
        $head
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::view::Pair::new($head, $crate::view::view![$($tail),+])
    );
}

pub use view;
