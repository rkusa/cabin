use std::fmt::{self, Write};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CountAction {
    Increment,
}

impl Action<u32> for CountAction {
    fn apply(self, state: u32) -> u32 {
        match self {
            CountAction::Increment => state + 1,
        }
    }
}

pub fn counter(count: u32) -> impl View<CountAction> {
    (
        div().content(format!("Count: {}", count)),
        button::<CountAction>()
            .on_click(CountAction::Increment)
            .content("incr"),
    )
}

// result of #[component]
pub fn counter_component(count: u32) -> impl View<()> {
    Component {
        state: count,
        render: counter,
        action: PhantomData,
    }
}

// By making this private, the conversion from View<A> to View<()> is the feature
// that ensures the usage of #[component]
struct Component<S, V: View<A>, A> {
    state: S,
    render: fn(S) -> V,
    action: PhantomData<A>,
}

impl<S, V: View<A>, A> View<()> for Component<S, V, A> {
    fn render(self, out: impl Write) -> fmt::Result {
        let view = (self.render)(self.state);
        view.render(out)
    }
}

#[allow(unused)]
fn handle_component(id: &str, state: &str, action: &str) {
    match id {
        "crate::counter" => {
            let before: u32 = serde_json::from_str(state).unwrap();
            let action: CountAction = serde_json::from_str(action).unwrap();
            let after = action.apply(before);
            let _component = counter(after);
            // TODO: rerender
            // let _ = component.render(out)
        }
        _ => panic!("unknown component with id `{}`", id),
    }
}

pub trait View<A = ()> {
    fn render(self, out: impl Write) -> fmt::Result;
}

pub trait Action<S> {
    fn apply(self, state: S) -> S;
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

impl<V1, V2, A> View<A> for (V1, V2)
where
    V1: View<A>,
    V2: View<A>,
{
    fn render(self, mut out: impl Write) -> fmt::Result {
        self.0.render(&mut out)?;
        self.1.render(&mut out)?;
        Ok(())
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

impl<'a, A> View<A> for &'a str {
    fn render(self, mut out: impl Write) -> fmt::Result {
        out.write_str(self)?;
        Ok(())
    }
}

impl<A> View<A> for String {
    fn render(self, out: impl Write) -> fmt::Result {
        View::<A>::render(self.as_str(), out)
    }
}

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

pub fn render(view: impl View<()>) -> Result<String, fmt::Error> {
    let mut result = String::new();
    view.render(&mut result)?;
    Ok(result)
}

impl<A> Default for HtmlTagBuilder<A> {
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
        let view = (counter_component(count), div());
        let html = render(view).unwrap();
        assert_eq!(html, "<div>Count: 42</div><button>incr</button><div/>");
    }
}
