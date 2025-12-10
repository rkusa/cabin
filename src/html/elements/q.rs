use std::borrow::Cow;

use super::blockquote::Cite;
use super::common::Common;
use super::global::Global;
use crate::View;
use crate::html::attributes::{Attributes, WithAttribute};
use crate::html::{Aria, Html};

/// The `q` element represents some phrasing content quoted from another source.
pub fn q(content: impl View) -> Html<marker::Q, ()> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    Html::new("q", (), content)
}

mod macros {
    #[macro_export]
    macro_rules! q {
        ($($x:tt)*) => {
            $crate::html::elements::q::q($crate::view![$($x)*])
        }
    }

    pub use q;
}

pub use macros::q;

pub mod marker {
    pub struct Q;
}

impl<A: Attributes> Q for Html<marker::Q, A> {}
impl<A: Attributes> Common for Html<marker::Q, A> {}
impl<A: Attributes> Global for Html<marker::Q, A> {}
impl<A: Attributes> Aria for Html<marker::Q, A> {}

/// The `q` element represents some phrasing content quoted from another source.
pub trait Q: WithAttribute {
    /// Link to the source of the quotation or more information about the edit.
    fn cite(self, src: impl Into<Cow<'static, str>>) -> Self::Output<Cite> {
        self.with_attribute(Cite(src.into()))
    }
}
