pub mod anchor;
pub mod aria;
pub mod body;
pub mod button;
pub mod dialog;
pub mod div;
pub mod form;
pub mod global;
pub mod head;
pub mod html;
pub mod input;
pub mod label;
pub mod link;
pub mod nav;
pub mod script;
pub mod span;
pub mod time;

pub mod old {
    #[macro_export]
    macro_rules! element {
        ($dollar:tt, $mod:ident, $name:ident) => {
            pub fn $name<V: $crate::view::View>(content: V) -> $crate::html::Html<V, ()> {
                $crate::html::Html::new(stringify!($name), content)
            }
        };
        ($dollar:tt, $mod:ident, $name:ident, $kind_mod:ident, $kind_type:ident) => {
            pub fn $name<V: $crate::view::View>(
                content: V,
            ) -> $crate::html::Html<V, $crate::html::elements::$kind_mod::$kind_type> {
                $crate::html::Html::new(stringify!($name), content)
            }
        };
    }

    #[macro_export]
    macro_rules! void_element {
        ($dollar:tt, $mod:ident, $name:ident, $kind_mod:ident, $kind_type:ident) => {
            pub fn $name() -> $crate::html::Html<(), $crate::html::elements::$kind_mod::$kind_type>
            {
                $crate::html::Html::new(stringify!($name), ())
            }
        };
    }

    element!($, __caption, caption);
    element!($, __col, col);
    element!($, __colgroup, colgroup);
    element!($, __fieldset, fieldset);
    element!($, __h1, h1);
    element!($, __h2, h2);
    element!($, __h3, h3);
    element!($, __h4, h4);
    element!($, __h5, h5);
    element!($, __h6, h6);
    element!($, __hgroup, hgroup);
    element!($, __li, li);
    element!($, __table, table);
    element!($, __tbody, tbody);
    element!($, __td, td);
    element!($, __tfoot, tfoot);
    element!($, __th, th);
    element!($, __thead, thead);
    element!($, __tr, tr);
    element!($, __ul, ul);
}
