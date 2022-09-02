use std::fmt::{self, Write};
use std::marker::PhantomData;

use crate::view::View;

pub fn div<A>() -> HtmlTagBuilder<A> {
    HtmlTagBuilder {
        tag: "div",
        ..Default::default()
    }
}

pub fn button<A>() -> HtmlTagBuilder<A> {
    HtmlTagBuilder {
        tag: "button",
        ..Default::default()
    }
}

pub struct HtmlTag<V, A> {
    tag: &'static str,
    content: V,
    action: PhantomData<A>,
}

pub struct HtmlTagBuilder<A = ()> {
    tag: &'static str,
    // TODO: get rid of Box
    on_click: Option<A>,
}

impl<A> HtmlTagBuilder<A> {
    // TODO: not available for all tags (e.g. only for buttons)
    pub fn on_click(mut self, action: A) -> HtmlTagBuilder<A> {
        self.on_click = Some(action);
        self
    }

    pub fn content<V: View<A>>(self, content: V) -> HtmlTag<V, A> {
        HtmlTag {
            tag: self.tag,
            content,
            action: PhantomData,
        }
    }
}

impl<V, A> View<A> for HtmlTag<V, A>
where
    V: View<A>,
{
    fn render(self, mut out: impl Write) -> fmt::Result {
        write!(&mut out, "<{}>", self.tag)?;
        self.content.render(&mut out)?;
        write!(&mut out, "</{}>", self.tag)?;
        Ok(())
    }
}

impl<A> View<A> for HtmlTagBuilder<A> {
    fn render(self, mut out: impl Write) -> fmt::Result {
        write!(out, "<{}/>", self.tag)
    }
}

impl<A> Default for HtmlTagBuilder<A> {
    fn default() -> Self {
        Self {
            tag: "div",
            on_click: None,
        }
    }
}
