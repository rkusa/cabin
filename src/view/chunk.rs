use crate::render::Renderer;
use crate::view::RenderFuture;
use crate::{Context, View};

pub struct ViewChunk {
    pub(crate) result: Result<Renderer, crate::Error>,
}

impl<'v> View<'v> for ViewChunk {
    fn render(self, _c: &'v Context, mut r: Renderer) -> RenderFuture<'v> {
        match self.result {
            Ok(mut renderer) => {
                r.append(&mut renderer);
                RenderFuture::ready(Ok(r))
            }
            Err(err) => RenderFuture::ready(Err(err)),
        }
    }
}
