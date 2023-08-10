use std::borrow::Cow;

use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use super::time::Datetime;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};
use crate::View;

/// The `del` element represents a removal from the document.
pub fn del(content: impl View) -> Html<marker::Del, (), impl View> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("del", (), content)
}

pub mod marker {
    pub struct Del;
}

impl<A: Attributes, V: 'static> Del for Html<marker::Del, A, V> {}
impl<A: Attributes, V: 'static> Common for Html<marker::Del, A, V> {}
impl<A: Attributes, V: 'static> Global for Html<marker::Del, A, V> {}
impl<A: Attributes, V: 'static> Aria for Html<marker::Del, A, V> {}

/// The `del` element represents a removal from the document.
pub trait Del: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Cite> {
        self.with_attribute(Cite(src.into()))
    }

    /// Machine-readable datetime/date/time of the change.
    fn datetime(self, datetime: impl Into<Cow<'static, str>>) -> Self::Output<Datetime> {
        self.with_attribute(Datetime(datetime.into()))
    }
}
