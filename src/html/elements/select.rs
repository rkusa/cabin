use std::borrow::Cow;

use super::button::Name;
use super::common::Common;
use super::global::Global;
use super::input::OnChange;
use crate::error::InternalError;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::events::InputEvent;
use crate::html::{Aria, Html};
use crate::View;

pub fn select(content: impl View) -> Html<marker::Select, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("select", (), content)
}

pub mod marker {
    pub struct Select;
}

impl<A: Attributes, V: 'static> Select for Html<marker::Select, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Select, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Select, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Select, A, V> {}

// TODO:
pub trait Select: WithAttribute {
    /// Name of the element to use for form submission.
    fn name(self, name: impl Into<Cow<'static, str>>) -> Self::Output<Name> {
        self.with_attribute(Name(name.into()))
    }

    fn on_change<E>(self, event: impl FnOnce(InputEvent) -> E) -> Self::Output<OnChange>
    where
        E: ::serde::Serialize + 'static,
    {
        let event = event(InputEvent::default());
        self.with_attribute(OnChange(Box::new(move || {
            use std::hash::{Hash, Hasher};

            let mut hasher = twox_hash::XxHash32::default();
            std::any::TypeId::of::<E>().hash(&mut hasher);
            let hash = hasher.finish() as u32;
            serde_json::to_string(&event)
                .map_err(|err| InternalError::Serialize {
                    what: "on_change event",
                    err,
                })
                .map(|json| (hash, json))
        })))
    }
}
