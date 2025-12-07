use std::cell::RefCell;
use std::rc::Rc;

use crate::render::Renderer;

#[derive(Clone)]
pub struct RendererPool {
    inner: Rc<RefCell<Inner>>,
    is_update: bool,
}

struct Inner {
    pool: Vec<Renderer>,
}

impl RendererPool {
    pub fn new(is_update: bool) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Inner {
                pool: Vec::with_capacity(8),
            })),
            is_update,
        }
    }

    pub fn is_update(&self) -> bool {
        self.is_update
    }

    pub fn acquire(&self) -> Renderer {
        let mut inner = self.inner.borrow_mut();
        inner
            .pool
            .pop()
            .unwrap_or_else(|| Renderer::new(self.clone()))
    }

    pub fn release(&self, mut renderer: Renderer) {
        renderer.reset();
        let mut inner = self.inner.borrow_mut();
        inner.pool.push(renderer);
    }

    pub fn drain(&self) {
        let mut inner = self.inner.borrow_mut();
        eprintln!("poll size: {}", inner.pool.len());
        inner.pool.clear();
    }
}
