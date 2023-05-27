use super::{create, Html};

pub mod anchor;
pub mod input;

#[macro_export]
macro_rules! element {
    ($dollar:tt, $mod:ident, $name:ident) => {
        pub fn $name<V: $crate::view::View>(content: V) -> $crate::html::Html<V, (), ()> {
            $crate::html::create(stringify!($name), content)
        }
    };
    ($dollar:tt, $mod:ident, $name:ident, $kind_mod:ident, $kind_type:ident) => {
        pub fn $name<V: $crate::view::View>(
            content: V,
        ) -> $crate::html::Html<V, (), $crate::html::elements::$kind_mod::$kind_type> {
            $crate::html::create(stringify!($name), content)
        }
    };
}

#[macro_export]
macro_rules! void_element {
    ($dollar:tt, $mod:ident, $name:ident, $kind_mod:ident, $kind_type:ident) => {
        pub fn $name() -> Html<(), (), $crate::html::elements::$kind_mod::$kind_type> {
            create(stringify!($name), ())
        }
    };
}

element!($, __a, a, anchor, Anchor);
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
void_element!($, __input, input, input, Input);
element!($, __li, li);
element!($, __table, table);
element!($, __tbody, tbody);
element!($, __td, td);
element!($, __tfoot, tfoot);
element!($, __th, th);
element!($, __thead, thead);
element!($, __tr, tr);
element!($, __ul, ul);
