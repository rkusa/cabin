use std::fmt;

use crate::html::attributes::Attributes;
use crate::html::{create, Html};
use crate::render::ElementRenderer;

pub fn script() -> Html<(), (), Script> {
    create("script", Script(()), ())
}

// TODO: allow children

pub struct Script(());
impl Attributes for Script {
    fn render(&self, _r: &mut ElementRenderer) -> Result<(), fmt::Error> {
        Ok(())
    }
}
