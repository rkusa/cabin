use std::borrow::Cow;

use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use super::time::Datetime;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `ins` element represents an addition to the document.
pub fn ins(content: impl View) -> Html<marker::Ins, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("ins", (), content)
}

pub mod marker {
    pub struct Ins;
}

impl<A: Attributes, V: 'static> Ins for Html<marker::Ins, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Ins, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Ins, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Ins, A, V> {}

/// The `ins` element represents an addition to the document.
pub trait Ins: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Cite> {
        self.with_attribute(Cite(src.into()))
    }

    /// Machine-readable datetime/date/time of the change.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self::Output<Datetime> {
        self.with_attribute(Datetime(datetime.into()))
    }
}
