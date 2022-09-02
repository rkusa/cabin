use std::fmt::{self, Write};
use std::marker::PhantomData;

use crate::View;

// The conversion from View<A> to View<()> is the feature
// that ensures the usage of #[component]
pub struct Component<S, V: View<A>, A> {
    state: S,
    render: fn(S) -> V,
    action: PhantomData<A>,
}

impl<S, V: View<A>, A> Component<S, V, A> {
    pub fn new(state: S, render: fn(S) -> V) -> Self {
        Component {
            state,
            render,
            action: PhantomData,
        }
    }
}

impl<S, V: View<A>, A> View<()> for Component<S, V, A> {
    fn render(self, out: impl Write) -> fmt::Result {
        let view = (self.render)(self.state);
        view.render(out)
    }
}
