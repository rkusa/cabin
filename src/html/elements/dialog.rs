use crate::html::{ElementExt, Html};
use crate::render::ElementRenderer;
use crate::View;

#[derive(Default)]
pub struct Dialog {
    open: bool,
}

impl<V> Html<V, Dialog>
where
    V: View,
{
    pub fn open(mut self, open: bool) -> Self {
        self.kind.open = open;
        self
    }
}

impl ElementExt for Dialog {
    fn render(self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        if self.open {
            r.attribute("open", "")
                .map_err(crate::error::InternalError::from)?;
        }

        Ok(())
    }
}
