use std::fmt::{self, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Count(u32);

#[derive(Serialize, Deserialize)]
pub enum CountAction {
    Increment,
}

impl Action for Count {
    type Action = CountAction;

    fn dispatch(self, action: Self::Action) -> Self {
        match action {
            CountAction::Increment => Count(self.0 + 1),
        }
    }
}

impl From<u32> for Count {
    fn from(count: u32) -> Self {
        Count(count)
    }
}

// #[component]
pub fn counter(count: impl Into<Count>) -> impl View {
    let count = count.into();
    (
        div().content(format!("Count: {}", count.0)),
        button::<CountAction>()
            .on_click(CountAction::Increment)
            .content("incr"),
    )
}

#[allow(unused)]
fn handle_component(id: &str, state: &str, action: &str) {
    match id {
        "crate::counter" => {
            let before: Count = serde_json::from_str(state).unwrap();
            let action: <Count as Action>::Action = serde_json::from_str(action).unwrap();
            let after = before.dispatch(action);
            let _component = counter(after);
            // TODO: rerender
            // let _ = component.render(out)
        }
        _ => panic!("unknown component with id `{}`", id),
    }
}

pub trait View {
    fn render(&self, out: impl Write) -> fmt::Result;
}

pub trait Action {
    type Action;
    fn dispatch(self, action: Self::Action) -> Self;
}

pub struct HtmlTag<V> {
    tag: &'static str,
    content: V,
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

pub fn button<A>() -> HtmlTagBuilder<A> {
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
        let view = (counter(count), div());
        let html = render(view).unwrap();
        assert_eq!(html, "<div>Count: 42</div><button>incr</button><div/>");
    }
}
