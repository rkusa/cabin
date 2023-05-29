use crate::html::attributes::Attributes;
use crate::html::Html;
use crate::render::ElementRenderer;
use crate::View;

#[derive(Default)]
pub struct Dialog {
    open: bool,
}

impl<V, Ev, A> Html<V, Ev, A, Dialog>
where
    V: View<Ev>,
{
    pub fn open(mut self, open: bool) -> Self {
        self.kind.open = open;
        self
    }
}

impl Attributes for Dialog {
    fn render(&self, r: &mut ElementRenderer) -> Result<(), crate::Error> {
        if self.open {
            r.attribute("open", "")
                .map_err(crate::error::InternalError::from)?;
        }

        Ok(())
    }
}