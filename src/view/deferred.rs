use super::boxed::BoxedView;
use crate::View;

pub trait DeferredView {
    fn view(self) -> BoxedView;
}

impl<D> View for D
where
    D: DeferredView + 'static,
{
    fn render(self, r: crate::render::Renderer, include_hash: bool) -> super::RenderFuture {
        self.view().render(r, include_hash)
    }
}
