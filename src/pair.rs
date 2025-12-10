use crate::View;
use crate::html::attributes::Attributes;
use crate::render::{ElementRenderer, Renderer};
use crate::scope::Scope;
use crate::view::RenderFuture;

pub struct Pair<L, R> {
    left: Option<L>,
    right: Option<R>,
}

impl<L, R> Pair<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Pair {
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn right(right: R) -> Self {
        Pair {
            left: None,
            right: Some(right),
        }
    }
}

impl<L: Attributes, R: Attributes> Attributes for Pair<L, R> {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        if let Some(left) = self.left {
            left.render(r)?
        };
        if let Some(right) = self.right {
            right.render(r)?
        };
        Ok(())
    }

    fn get<T: 'static>(&self) -> Option<&T> {
        self.left
            .as_ref()
            .and_then(|l| l.get())
            .or_else(|| self.right.as_ref().and_then(|r| r.get()))
    }

    fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.left
            .as_mut()
            .and_then(|l| l.get_mut())
            .or_else(|| self.right.as_mut().and_then(|r| r.get_mut()))
    }
}

impl<L: View, R: View> Pair<L, R> {
    fn render_left(&mut self, r: Renderer) -> RenderFuture {
        if let Some(left) = self.left.take() {
            left.render(r)
        } else {
            RenderFuture::Ready(Ok(r))
        }
    }

    fn render_right(&mut self, r: Renderer) -> RenderFuture {
        if let Some(right) = self.right.take() {
            right.render(r)
        } else {
            RenderFuture::Ready(Ok(r))
        }
    }
}

impl<L: View, R: View> View for Pair<L, R> {
    fn render(mut self, r: Renderer) -> RenderFuture {
        match self.render_left(r) {
            RenderFuture::Ready(Ok(r)) => self.render_right(r),
            RenderFuture::Ready(Err(err)) => RenderFuture::Ready(Err(err)),
            RenderFuture::Future(fut) => RenderFuture::Future(Box::pin(async move {
                let (mut l, r) = futures_util::future::try_join(
                    fut,
                    self.render_right(Scope::create_renderer_from_task()),
                )
                .await?;
                l.append(r);
                Ok(l)
            })),
        }
    }
}
