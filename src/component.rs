use std::fmt::{self, Write};
use std::marker::PhantomData;

use serde::Serialize;

use crate::html::{self, HtmlTagBuilder};
use crate::View;

// The conversion from View<A> to View<()> is the feature
// that ensures the usage of #[component]
pub struct Component<S, V: View<A>, A> {
    id: &'static str,
    state: S,
    render: fn(S) -> V,
    action: PhantomData<A>,
}

impl<S, V: View<A>, A> Component<S, V, A> {
    pub fn new(id: &'static str, state: S, render: fn(S) -> V) -> Self {
        Component {
            id,
            state,
            render,
            action: PhantomData,
        }
    }

    pub fn render_update(self) -> Result<String, fmt::Error> {
        let mut result = String::new();
        let view = (self.render)(self.state);
        view.render(&mut result)?;
        Ok(result)
    }
}

impl<S: Serialize, V: View<A>, A: Serialize> View<()> for Component<S, V, A> {
    fn render(self, out: impl Write) -> fmt::Result {
        // TODO: unwrap
        let state = serde_json::to_string(&self.state).unwrap();
        let view = (self.render)(self.state);
        let view = HtmlTagBuilder::new("server-component").content((
            html::custom("script")
                .attr("type", "application/json")
                .content(state),
            view,
        ));
        view.render(out)
    }
}
