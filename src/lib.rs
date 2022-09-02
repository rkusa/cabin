#![feature(type_alias_impl_trait)]

use std::borrow::Cow;
use std::fmt::{self, Write};

use serde::Serialize;

// pub trait Component {
//     type State: Serialize;
//     type Action: Serialize;
//     type View: View;

//     fn render(state: &Self::State) -> Self::View;
//     fn dispatch(state: Self::State, action: Self::Action) -> Self::State;
// }

pub struct Component<S, V> {
    state: S,
    view: fn(state: &S) -> V,
}

// result of #[component]
mod _counter {
    use super::*;
    type ComponentView = impl crate::View;
    pub type ComponentState = u32;
    fn component(count: &ComponentState) -> ComponentView {
        (
            div().content(format!("Count: {}", *count)),
            button(count)
                // .on_click(|count| count + 1)
                .content("incr"),
        )
    }
    pub fn counter(count: ComponentState) -> Component<ComponentState, ComponentView> {
        Component::new(count, component)
    }
}
pub use _counter::counter;

fn handle_component(id: &str, input: &str) {
    match id {
        "crate::counter" => {
            let state: _counter::ComponentState = serde_json::from_str(input).unwrap();
            let _component = _counter::counter(state);
            // TODO: apply action
            // TODO: rerender
            // let _ = component.render(out)
        }
        _ => panic!("unknown component with id `{}`", id),
    }
}

pub trait View {
    fn render(&self, out: impl Write) -> fmt::Result;
}

impl<S, V> Component<S, V> {
    fn new(state: S, view: fn(state: &S) -> V) -> Self {
        Component { state, view }
    }
}

impl<S, V> View for Component<S, V>
where
    S: Serialize,
    V: View,
{
    fn render(&self, out: impl Write) -> fmt::Result {
        (self.view)(&self.state).render(out)
    }
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
        let count = 42;
        let view = (counter(count), div());
        let html = render(view).unwrap();
        assert_eq!(html, "<div>Count: 42</div><button>incr</button><div/>");
    }
}
