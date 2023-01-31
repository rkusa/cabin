use super::{create, Html};

mod anchor;
mod input;

#[macro_export]
macro_rules! element {
    ($dollar:tt, $mod:ident, $name:ident) => {
        element!($dollar, $mod, $name, ());
    };
    ($dollar:tt, $mod:ident, $name:ident, $kind:ty) => {
        pub fn $name<V: $crate::view::View>(content: impl $crate::view::IntoView<V>) -> $crate::html::Html<V, (), $kind> {
            $crate::html::create(stringify!($name), content.into_view())
        }

        mod $mod {
            #[macro_export]
            macro_rules! $name {
                ($dollar($x:tt)*) => ($crate::html::custom(stringify!($name), $crate::view![$dollar($x)*]))
            }

            pub use $name;
        }
        pub use $mod::*;
    };
}

#[macro_export]
macro_rules! void_element {
    ($dollar:tt, $mod:ident, $name:ident, $kind:ty) => {
        pub fn $name() -> Html<(), (), $kind> {
            create(stringify!($name), ())
        }
    };
}

element!($, __a, a, anchor::Anchor);
element!($, __button, button);
element!($, __caption, caption);
element!($, __col, col);
element!($, __colgroup, colgroup);
element!($, __div, div);
element!($, __fieldset, fieldset);
element!($, __h1, h1);
element!($, __h2, h2);
element!($, __h3, h3);
element!($, __h4, h4);
element!($, __h5, h5);
element!($, __h6, h6);
element!($, __hgroup, hgroup);
void_element!($, __input, input, input::Input);
element!($, __li, li);
element!($, __table, table);
element!($, __tbody, tbody);
element!($, __td, td);
element!($, __tfoot, tfoot);
element!($, __th, th);
element!($, __thead, thead);
element!($, __tr, tr);
element!($, __ul, ul);
