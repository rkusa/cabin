use crate::View;
use crate::render::Renderer;
use crate::scope::Scope;
use crate::view::RenderFuture;

pub struct AnyView {
    view: RenderFuture,
}

impl AnyView {
    #[inline]
    pub fn new(view: impl View) -> Self {
        let r = Scope::create_renderer_from_task();
        Self {
            view: view.render(r),
        }
    }
}

impl View for AnyView {
    fn render(self, mut r: Renderer) -> RenderFuture {
        match self.view {
            RenderFuture::Ready(Ok(inner)) => {
                r.append(inner);
                RenderFuture::Ready(Ok(r))
            }
            RenderFuture::Ready(Err(err)) => RenderFuture::Ready(Err(err)),
            RenderFuture::Future(fut) => RenderFuture::Future(Box::pin(async move {
                let inner = fut.await?;
                r.append(inner);
                Ok(r)
            })),
        }
    }
}
