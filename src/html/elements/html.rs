use crate::View;
use crate::html::attributes::Attributes;
use crate::html::{Global, Html};
use crate::view::UpdateView;

/// The `html` element represents the root of an HTML document.
pub fn html(content: impl View) -> UpdateView<Html<marker::Html, ()>> {
    #[cfg(debug_assertions)]
    let content = content.boxed();
    UpdateView::content_only_on_update(Html::new("html", (), content))
}

mod macros {
    #[macro_export]
    macro_rules! html {
        ($($x:tt)*) => {
            $crate::html::elements::html::html($crate::view![$($x)*])
        }
    }

    pub use html;
}

pub use macros::html;

pub mod marker {
    pub struct Html;
}

impl<A: Attributes> Global for UpdateView<Html<marker::Html, A>> {}
