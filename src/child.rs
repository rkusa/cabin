use crate::view::IntoView;

pub trait IntoChild<C> {
    fn into_child(self) -> C;
}

impl<'v, V> IntoChild<V> for V
where
    V: IntoView<'v>,
{
    fn into_child(self) -> V {
        self
    }
}
