use crate::View;
use crate::render::Renderer;
use crate::scope::Scope;
use crate::view::RenderFuture;

pub struct AnyView {
    pub(crate) view: RenderFuture,
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

impl Future for AnyView {
    type Output = Self;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match self.get_mut().view {
            RenderFuture::Ready(ref mut result) => {
                let result = std::mem::replace(
                    result,
                    Err(crate::error::InternalError::FutureCompleted.into()),
                );
                std::task::Poll::Ready(AnyView {
                    view: RenderFuture::Ready(result),
                })
            }
            RenderFuture::Future(ref mut future) => future.as_mut().poll(cx).map(|view| AnyView {
                view: RenderFuture::Ready(view),
            }),
        }
    }
}
