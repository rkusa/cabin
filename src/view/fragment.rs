use std::ops::Shr;

use crate::view::Pair;
use crate::View;

pub struct Fragment<V> {
    inner: V,
}

pub fn fragment() -> Fragment<()> {
    Fragment { inner: () }
}

impl<V: View> View for Fragment<V> {
    type Future = V::Future;

    fn render(self, r: crate::Renderer) -> Self::Future {
        self.inner.render(r)
    }
}

impl<Lhs: View, Rhs: View> Shr<Rhs> for Fragment<Lhs> {
    type Output = Fragment<Pair<Lhs, Rhs>>;

    fn shr(self, rhs: Rhs) -> Self::Output {
        Fragment {
            inner: Pair::new(self.inner, rhs),
        }
    }
}
