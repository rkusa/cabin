use std::fmt::{self, Write};

pub trait View<S> {
    fn render(&self, state: &S, out: impl Write) -> fmt::Result;
}

pub struct HtmlTag<V> {
    tag: &'static str,
    content: V,
}

pub struct HtmlTagBuilder<S = ()> {
    tag: &'static str,
    // TODO: get rid of Box
    on_click: Option<Box<dyn FnOnce(S) -> S + 'static>>,
}

impl<S> HtmlTagBuilder<S> {
    // TODO: not available for all tags (e.g. only for buttons)
    pub fn on_click(mut self, handler: impl FnOnce(S) -> S + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn content<V: View<S>>(self, content: V) -> HtmlTag<V> {
        HtmlTag {
            tag: self.tag,
            content,
        }
    }
}

impl<S1, S2, V1, V2> View<(S1, S2)> for (V1, V2)
where
    V1: View<S1>,
    V2: View<S2>,
{
    fn render(&self, state: &(S1, S2), mut out: impl Write) -> fmt::Result {
        self.0.render(&state.0, &mut out)?;
        self.1.render(&state.1, &mut out)?;
        Ok(())
    }
}

impl<S, V> View<S> for HtmlTag<V>
where
    V: View<S>,
{
    fn render(&self, state: &S, mut out: impl Write) -> fmt::Result {
        write!(&mut out, "<{}>", self.tag)?;
        self.content.render(state, &mut out)?;
        write!(&mut out, "</{}>", self.tag)?;
        Ok(())
    }
}

impl<S> View<S> for HtmlTagBuilder {
    fn render(&self, _state: &S, mut out: impl Write) -> fmt::Result {
        write!(out, "<{}/>", self.tag)
    }
}

impl<'a, S> View<S> for &'a str {
    fn render(&self, _state: &S, mut out: impl Write) -> fmt::Result {
        out.write_str(self)?;
        Ok(())
    }
}

impl<S> View<S> for String {
    fn render(&self, state: &S, out: impl Write) -> fmt::Result {
        self.as_str().render(state, out)
    }
}

pub fn div() -> HtmlTagBuilder {
    HtmlTagBuilder {
        tag: "div",
        ..Default::default()
    }
}

pub fn button<S>(state: S) -> HtmlTagBuilder<S> {
    HtmlTagBuilder {
        tag: "button",
        ..Default::default()
    }
}

pub fn render(view: impl View<()>) -> Result<String, fmt::Error> {
    let mut result = String::new();
    view.render(&(), &mut result)?;
    Ok(result)
}

// #[derive(Default)]
// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new(count: u32) -> Self {
//         Self { count }
//     }
// }

// impl<S> View<S> for Counter {
//     fn render(&self, state: &S, out: impl Write) -> fmt::Result {
//         (
//             format!("Count: {}", count),
//             button()
//                 .on_click(|| self.count + 1)
//                 .content::<(), _>("incr"),
//         )
//             .render(state, out)
//     }
// }

fn counter(count: u32) -> impl View<u32> {
    (
        format!("Count: {}", count),
        button(count).on_click(|count| count + 1).content("incr"),
    )
}

impl<S> Default for HtmlTagBuilder<S> {
    fn default() -> Self {
        Self {
            tag: "div",
            on_click: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let count = 42;
        let view = div().content((counter(count), div()));
        let html = render(view).unwrap();
        assert_eq!(html, "<div>Count: 42<div/></div>");
    }
}
