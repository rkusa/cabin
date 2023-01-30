mod input;
mod scripting;
mod tabular_data;

use std::fmt;

pub use input::*;
pub use scripting::*;
pub use tabular_data::*;

use crate::render::ElementRenderer;
use crate::view::Pair;
use crate::{IntoView, View};

use super::attributes::Attributes;
use super::{create, AddChild, Html};

pub fn div<V: View>(content: impl IntoView<V>) -> Html<V, (), Flow> {
    create("div", Flow(()), content.into_view())
}

pub fn ul<V: View>(content: impl IntoView<V>) -> Html<V, (), Flow> {
    create("ul", Flow(()), content.into_view())
}

pub fn li<V: View>(content: impl IntoView<V>) -> Html<V, (), Flow> {
    create("li", Flow(()), content.into_view())
}

pub fn fieldset<V: View>(content: impl IntoView<V>) -> Html<V, (), Flow> {
    create("fieldset", Flow(()), content.into_view())
}

pub fn button<V: View>(content: impl IntoView<V>) -> Html<V, (), Flow> {
    create("button", Flow(()), content.into_view())
}

pub struct Flow(());
impl Attributes for Flow {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}

impl<C, V, A> AddChild<C> for Html<V, A, Flow>
where
    V: View + Send + 'static,
    C: View + Send + 'static,
{
    type Output = Html<Pair<V, C>, A, Flow>;

    fn add_child(self, child: C) -> Self::Output {
        Html {
            tag: self.tag,
            attrs: self.attrs,
            on_click: self.on_click,
            kind: self.kind,
            content: Pair::new(self.content, child),
        }
    }
}
