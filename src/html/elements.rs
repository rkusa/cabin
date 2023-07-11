use crate::render::ElementRenderer;

pub mod anchor;
pub mod aria;
pub mod body;
pub mod button;
pub mod dialog;
pub mod div;
pub mod fieldset;
pub mod form;
pub mod global;
pub mod head;
pub mod html;
pub mod input;
pub mod label;
pub mod li;
pub mod link;
pub mod nav;
pub mod script;
pub mod span;
pub mod time;
pub mod ul;

pub trait ElementExt {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error>;
}

pub trait Element: ElementExt {
    const TAG: &'static str;

    fn is_void_element() -> bool {
        false
    }
}

impl ElementExt for () {
    fn render(self, _r: &mut ElementRenderer) -> Result<(), crate::Error> {
        Ok(())
    }
}
