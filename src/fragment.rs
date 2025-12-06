use std::pin::Pin;

use crate::View;
use crate::child::IntoChild;
use crate::context::Context;
use crate::render::Renderer;
use crate::view::chunk::ViewChunk;
use crate::view::{IntoView, RenderFuture};

pub struct Fragment<'v> {
    previous_chunks: Vec<Pin<Box<dyn Future<Output = Result<Renderer, crate::Error>> + 'v>>>,
    renderer: Renderer,
    context: &'v Context,
    error: Option<crate::Error>,
}

impl<'v> Fragment<'v> {
    pub(crate) fn new(renderer: Renderer, context: &'v Context) -> Self {
        Self {
            previous_chunks: Vec::new(),
            renderer,
            context,
            error: None,
        }
    }

    pub fn child<V: IntoView<'v>>(mut self, child: impl IntoChild<V>) -> Self {
        if self.error.is_some() {
            return self;
        }

        let renderer = std::mem::replace(&mut self.renderer, self.context.acquire_renderer());
        match child
            .into_child()
            .into_view()
            .render(self.context, renderer)
        {
            RenderFuture::Ready(Some(Ok(renderer))) => {
                let other = std::mem::replace(&mut self.renderer, renderer);
                self.context.release_renderer(other);
            }
            RenderFuture::Ready(Some(Err(err))) => {
                self.error = Some(err);
            }
            RenderFuture::Ready(None) => {}
            RenderFuture::Future(future) => {
                self.previous_chunks.push(future);
            }
        }

        self
    }

    pub async fn finish(self) -> ViewChunk {
        let c = self.context;
        let r = c.acquire_renderer();
        ViewChunk {
            result: self.render(c, r).await,
        }
    }

    pub(crate) fn render_self(mut self) -> RenderFuture<'v> {
        if let Some(err) = self.error {
            return RenderFuture::ready(Err(err));
        }

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
    fn render(mut self, _c: &'v Context, mut r: Renderer) -> RenderFuture<'v> {
        if let Some(err) = self.error {
            return RenderFuture::ready(Err(err));
        }

        if self.previous_chunks.is_empty() {
            r.append(&mut self.renderer);
            self.context.release_renderer(self.renderer);
            RenderFuture::ready(Ok(r))
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

                r.append(&mut self.renderer);
                self.context.release_renderer(self.renderer);

                Ok(r)
            }))
        }
    }
}
