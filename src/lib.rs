use std::borrow::Cow;
use std::fmt::{self, Write};

use serde::Serialize;

pub const fn create_component<S, V>(
    state: S,
    component: impl Fn(&S) -> V,
) -> impl Component<View = V>
where
    S: Serialize,
    V: View,
{
    ComponentFn {
        state,
        component_fn: component,
    }
}

pub trait Component {
    type View: View;
    fn render(&self) -> Self::View;
}

struct ComponentFn<S, V, F>
where
    S: Serialize,
    V: View,
    F: Fn(&S) -> V,
{
    state: S,
    component_fn: F,
}

impl<S, V, F> Component for ComponentFn<S, V, F>
where
    S: Serialize,
    V: View,
    F: Fn(&S) -> V,
{
    type View = V;

    fn render(&self) -> Self::View {
        (self.component_fn)(&self.state)
    }
}

pub trait View {
    fn render(&self, out: impl Write) -> fmt::Result;
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

impl HtmlTagBuilder {
    // TODO: not available for all tags (e.g. only for buttons)
    // pub fn on_click(mut self, handler: impl FnOnce(S) -> S + 'static) -> Self {
    //     self.on_click = Some(Box::new(handler));
    //     self
    // }

    pub fn content<V: View>(self, content: V) -> HtmlTag<V> {
        HtmlTag {
            tag: self.tag,
            content,
        }
    }
}

impl<V1, V2> View for (V1, V2)
where
    V1: View,
    V2: View,
{
    fn render(&self, mut out: impl Write) -> fmt::Result {
        self.0.render(&mut out)?;
        self.1.render(&mut out)?;
        Ok(())
    }
}

impl<V> View for HtmlTag<V>
where
    V: View,
{
    fn render(&self, mut out: impl Write) -> fmt::Result {
        write!(&mut out, "<{}>", self.tag)?;
        self.content.render(&mut out)?;
        write!(&mut out, "</{}>", self.tag)?;
        Ok(())
    }
}

impl View for HtmlTagBuilder {
    fn render(&self, mut out: impl Write) -> fmt::Result {
        write!(out, "<{}/>", self.tag)
    }
}

impl<'a> View for &'a str {
    fn render(&self, mut out: impl Write) -> fmt::Result {
        out.write_str(self)?;
        Ok(())
    }
}

impl View for String {
    fn render(&self, out: impl Write) -> fmt::Result {
        self.as_str().render(out)
    }
}

pub fn div() -> HtmlTagBuilder {
    HtmlTagBuilder {
        tag: "div",
        ..Default::default()
    }
}

pub fn button<S>(state: S) -> HtmlTagBuilder {
    HtmlTagBuilder {
        tag: "button",
        ..Default::default()
    }
}

pub fn render(view: impl View) -> Result<String, fmt::Error> {
    let mut result = String::new();
    view.render(&mut result)?;
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

// #[component]
fn counter(count: &u32) -> impl View {
    // const counter: Box<dyn Component> = Box::new(create_component(0, |count: &u32| {
    (
        div().content(format!("Count: {}", *count)),
        button(count)
            // .on_click(|count| count + 1)
            .content("incr"),
    )
}

impl Default for HtmlTagBuilder {
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
        let _component = create_component(0, counter);

        let count = 42;
        let view = (counter(&count), div());
        let html = render(view).unwrap();
        assert_eq!(html, "<div>Count: 42</div><button>incr</button><div/>");
    }
}
