use std::fmt;

use crate::html::attributes::Attributes;
use crate::html::{create, Html};
use crate::render::ElementRenderer;

pub fn template() -> Html<(), (), Template> {
    create("template", Template(()), ())
}

// TODO: allow children

pub struct Template(());
impl Attributes for Template {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}
