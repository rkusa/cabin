use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::view::UpdateView;

/// The `head` element represents a collection of metadata for the document.
pub fn head(content: impl View) -> UpdateView<Html<marker::Head, ()>> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    UpdateView::hidden_on_update(Html::new("head", (), content))
}

mod macros {
    #[macro_export]
    macro_rules! head {
        ($($x:tt)*) => {
            $crate::html::elements::head::head($crate::view![$($x)*])
        }
    }

    pub use head;
}

pub use macros::head;

pub mod marker {
    pub struct Head;
}

impl<A: Attributes> Global for UpdateView<Html<marker::Head, A>> {}
