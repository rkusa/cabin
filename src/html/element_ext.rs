use crate::render::ElementRenderer;

pub trait ElementExt {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error>;
}

impl ElementExt for () {
    fn render(self, _r: &mut ElementRenderer) -> Result<(), crate::Error> {
        Ok(())
    }
}
