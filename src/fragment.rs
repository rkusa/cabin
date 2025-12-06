use std::pin::Pin;

use crate::View;
use crate::context::Context;
use crate::render::Renderer;
use crate::view::chunk::ViewChunk;
use crate::view::internal::{Internal, Render};
use crate::view::{IntoView, RenderFuture};

pub struct Fragment<'v>(Internal<'v, FragmentBuilder<'v>>);

struct FragmentBuilder<'v> {
    previous_chunks: Vec<Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + 'v>>>,
    renderer: Renderer,
    context: &'v Context,
}

impl<'v> Fragment<'v> {
    pub(crate) fn new(renderer: Renderer, context: &'v Context) -> Self {
        Self(Internal::new(FragmentBuilder {
            previous_chunks: Vec::new(),
            renderer,
            context,
        }))
    }

    pub fn child(mut self, child: impl IntoView<'v>) -> Self {
        self.append_child(child);
        self
    }

    pub(crate) fn append_child(&mut self, child: impl IntoView<'v>) {
        let Some(builder) = self.0.builder_mut() else {
            return;
        };

        let renderer = std::mem::replace(&mut builder.renderer, builder.context.acquire_renderer());
        match child.into_view().render(builder.context, renderer) {
            RenderFuture::Ready(Some(Ok(renderer))) => {
                let other = std::mem::replace(&mut builder.renderer, renderer);
                builder.context.release_renderer(other);
            }
            RenderFuture::Ready(Some(Err(err))) => {
                self.0.errored(err);
            }
            RenderFuture::Ready(None) => {}
            RenderFuture::Future(future) => {
                builder.previous_chunks.push(future);
            }
        }
    }

    pub(crate) fn render(self) -> RenderFuture<'v> {
        self.0.render()
    }
}

impl<'v> Render<'v> for FragmentBuilder<'v> {
    fn render(mut self) -> RenderFuture<'v> {
        if self.previous_chunks.is_empty() {
            RenderFuture::ready(Ok(self.renderer))
        } else {
            RenderFuture::Future(Box::pin(async move {
                let results = futures_util::future::join_all(self.previous_chunks).await;
                for result in results {
                    match result {
                        Ok(mut renderer) => {
                            self.renderer.append(&mut renderer);
                            self.context.release_renderer(renderer);
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }

                Ok(self.renderer)
            }))
        }
    }
}

impl<'v> View<'v> for Fragment<'v> {
    fn render(self, c: &'v Context, r: Renderer) -> RenderFuture<'v> {
        self.0.render().merge_into(c, r)
    }
}

impl<'v> Future for Fragment<'v> {
    type Output = ViewChunk;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::pin::pin!(&mut self.get_mut().0).as_mut().poll(cx)
    }
}
