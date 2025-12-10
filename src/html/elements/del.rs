use std::borrow::Cow;

use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use super::time::Datetime;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `del` element represents a removal from the document.
pub fn del(content: impl View) -> Html<marker::Del, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("del", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! del {
        ($($x:tt)*) => {
            $crate::html::elements::del::del($crate::view![$($x)*])
        }
    }

    pub use del;
}

pub use macros::del;

pub mod marker {
    pub struct Del;
}

impl<A: Attributes> Del for Html<marker::Del, A> {}
impl<A: Attributes> Common for Html<marker::Del, A> {}
impl<A: Attributes> Global for Html<marker::Del, A> {}
impl<A: Attributes> Aria for Html<marker::Del, A> {}

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
