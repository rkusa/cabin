use crate::view::Pair;
use crate::{Renderer, View};

use super::AddChild;

pub struct Fragment<V> {
    content: V,
}

impl<C, V> AddChild<C> for Fragment<V>
where
    V: View + Send + 'static,
    C: View + Send + 'static,
{
    type Output = Fragment<Pair<V, C>>;

    fn add_child(self, child: C) -> Self::Output {
        Fragment {
            content: Pair::new(self.content, child),
        }
    }
}

impl Default for Fragment<()> {
    fn default() -> Self {
        Self { content: () }
    }
}

impl<V> View for Fragment<V>
where
    // TODO: remove `+ 'static` once removing away from boxed future
    V: View + Send + 'static,
{
    type Future = V::Future;

    fn render(self, r: Renderer) -> Self::Future {
        self.content.render(r)
    }
}
