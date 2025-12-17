#[macro_export]
macro_rules! view {
    () => (
        ()
    );
    ($head:expr $(,)?) => (
        $head
    );
    ($head:expr, $($tail:expr),+ $(,)?) => (
        $crate::view::AnyView::new($head) $(.appended($tail))+
    );
}

pub use view;
