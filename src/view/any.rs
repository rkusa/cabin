use smallvec::SmallVec;

use crate::html::Common;
use crate::render::Renderer;
use crate::scope::Scope;
use crate::view::RenderFuture;
use crate::{View, h};

pub struct AnyView {
    pub(crate) views: SmallVec<RenderFuture, 1>,
}

impl AnyView {
    #[inline]
    pub fn new(view: impl View) -> Self {
        let r = Scope::create_renderer_from_task();
        Self {
            views: smallvec::smallvec![view.render(r)],
        }
    }

    pub async fn collect_styles(self, is_page_style: bool) -> (Self, impl View) {
        let r = Scope::create_renderer_from_task();
        match self.render(r).await {
            Ok(mut r) => {
                let css = r.build_styles(is_page_style);
                (
                    Self {
                        views: smallvec::smallvec![RenderFuture::Ready(Ok(r))],
                    },
                    if is_page_style {
                        h::style(css).id("cabin-styles").boxed()
                    } else {
                        h::style(css).boxed()
                    },
                )
            }
            Err(err) => (
                Self {
                    views: smallvec::smallvec![RenderFuture::Ready(Err(err))],
                },
                h::style("").boxed(),
            ),
        }
    }

    pub fn appended(mut self, other: impl View) -> Self {
        let Some(last) = self.views.last_mut() else {
            return self;
        };
        match std::mem::replace(
            last,
            RenderFuture::Ready(Err(crate::error::InternalError::FutureCompleted.into())),
        ) {
            RenderFuture::Ready(Ok(r)) => {
                *last = other.render(r);
            }
            RenderFuture::Ready(Err(err)) => {
                *last = RenderFuture::Ready(Err(err));
            }
            RenderFuture::Future(fut) => {
                *last = RenderFuture::Future(fut);
                self.views
                    .push(other.render(Scope::create_renderer_from_task()));
            }
        }
        self
    }
}

impl View for AnyView {
    fn render(self, mut r: Renderer) -> RenderFuture {
        let mut views = self.views.into_iter();
        loop {
            let Some(view) = views.next() else {
                break;
            };
            match view {
                RenderFuture::Ready(Ok(inner)) => {
                    r.append(inner);
                }
                RenderFuture::Ready(Err(err)) => return RenderFuture::Ready(Err(err)),
                RenderFuture::Future(fut) => {
                    return RenderFuture::Future(Box::pin(async move {
                        let childs = futures_util::future::try_join_all(
                            std::iter::once(RenderFuture::Future(fut)).chain(views),
                        )
                        .await?;
                        for c in childs {
                            r.append(c);
                        }
                        Ok(r)
                    }));
                }
            }
        }
        RenderFuture::Ready(Ok(r))
    }
}

impl Future for AnyView {
    type Output = Self;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let self_mut = self.get_mut();
        let mut all_ready = true;
        for view in &mut self_mut.views {
            match view {
                RenderFuture::Ready(Err(err)) => {
                    let err =
                        std::mem::replace(err, crate::error::InternalError::FutureCompleted.into());
                    return std::task::Poll::Ready(AnyView {
                        views: smallvec::smallvec![RenderFuture::Ready(Err(err))],
                    });
                }
                RenderFuture::Ready(Ok(_)) => {
                    continue;
                }
                RenderFuture::Future(future) => match future.as_mut().poll(cx) {
                    std::task::Poll::Ready(Err(err)) => {
                        return std::task::Poll::Ready(AnyView {
                            views: smallvec::smallvec![RenderFuture::Ready(Err(err))],
                        });
                    }
                    std::task::Poll::Ready(Ok(r)) => {
                        *view = RenderFuture::Ready(Ok(r));
                    }
                    std::task::Poll::Pending => {
                        all_ready = false;
                        // continue with all child views to ensure all of them get polled in parallel
                        continue;
                    }
                },
            }
        }
        if all_ready {
            std::task::Poll::Ready(AnyView {
                views: std::mem::take(&mut self_mut.views),
            })
        } else {
            std::task::Poll::Pending
        }
    }
}
