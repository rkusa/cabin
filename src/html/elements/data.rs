use std::borrow::Cow;

use super::common::Common;
use super::global::Global;
use super::input::Value;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `data` element represents its contents, along with a machine-readable form of those
/// contents in the `value` attribute.
#[crate::view_macro(crate::html::elements::data)]
pub fn data(content: impl View) -> Html<marker::Data, ()> {
    Html::new("data", (), content)
}

pub mod marker {
    pub struct Data;
}

impl<A: Attributes> Data for Html<marker::Data, A> {}
impl<A: Attributes> Common for Html<marker::Data, A> {}
impl<A: Attributes> Global for Html<marker::Data, A> {}
impl<A: Attributes> Aria for Html<marker::Data, A> {}

/// The `data` element represents its contents, along with a machine-readable form of those
/// contents in the `value` attribute.
pub trait Data: WithAttribute {
    /// Machine-readable value.
    fn value(self, value: impl Into<Cow<'static, str>>) -> Self::Output<Value> {
        self.with_attribute(Value(value.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Renderer;

    #[tokio::test]
    async fn test_data() {
        assert_eq!(
            data("Mini Ketchup")
                .value("398")
                .render(Renderer::new(false, true))
                .await
                .unwrap()
                .end()
                .html,
            r#"<data value="398">Mini Ketchup</data>"#
        );
    }
}
